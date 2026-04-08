use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{Group, GroupInvitation, GroupJoinRequest, GroupMember, GroupRole, InvitationStatus, JoinRequestStatus};

pub struct GroupRepository {
    pool: PgPool,
}

impl GroupRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_group(
        &self,
        name: &str,
        slug: &str,
        description: Option<&str>,
        creator_id: Uuid,
        is_private: bool,
    ) -> Result<Group, AppError> {
        let group = sqlx::query_as::<_, Group>(
            r#"
            INSERT INTO groups (name, slug, description, creator_id, is_private)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(description)
        .bind(creator_id)
        .bind(is_private)
        .fetch_one(&self.pool)
        .await?;

        // Add creator as admin
        self.add_member(group.id, creator_id, GroupRole::Admin).await?;

        Ok(group)
    }

    pub async fn get_group_by_id(&self, group_id: Uuid) -> Result<Option<Group>, AppError> {
        let group = sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE id = $1")
            .bind(group_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(group)
    }

    #[allow(dead_code)]
    pub async fn get_group_by_slug(&self, slug: &str) -> Result<Option<Group>, AppError> {
        let group = sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE slug = $1")
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;

        Ok(group)
    }

    pub async fn list_groups(
        &self,
        offset: i32,
        limit: i32,
        user_id: Option<Uuid>,
    ) -> Result<Vec<Group>, AppError> {
        let groups = if let Some(uid) = user_id {
            // Show groups user is a member of + public groups
            sqlx::query_as::<_, Group>(
                r#"
                SELECT g.* FROM groups g
                LEFT JOIN group_members gm ON g.id = gm.group_id AND gm.user_id = $3
                WHERE (gm.user_id IS NOT NULL OR g.is_private = false)
                ORDER BY g.member_count DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit)
            .bind(offset)
            .bind(uid)
            .fetch_all(&self.pool)
            .await?
        } else {
            // Show only public groups
            sqlx::query_as::<_, Group>(
                r#"
                SELECT * FROM groups
                WHERE is_private = false
                ORDER BY member_count DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(groups)
    }

    pub async fn update_group(
        &self,
        group_id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        is_private: Option<bool>,
    ) -> Result<Group, AppError> {
        let group = sqlx::query_as::<_, Group>(
            r#"
            UPDATE groups
            SET 
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                is_private = COALESCE($4, is_private),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(group_id)
        .bind(name)
        .bind(description)
        .bind(is_private)
        .fetch_one(&self.pool)
        .await?;

        Ok(group)
    }

    pub async fn delete_group(&self, group_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM groups WHERE id = $1")
            .bind(group_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_member(
        &self,
        group_id: Uuid,
        user_id: Uuid,
        role: GroupRole,
    ) -> Result<GroupMember, AppError> {
        let member = sqlx::query_as::<_, GroupMember>(
            r#"
            INSERT INTO group_members (group_id, user_id, role)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(group_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(&self.pool)
        .await?;

        // Update member count
        sqlx::query(
            "UPDATE groups SET member_count = member_count + 1 WHERE id = $1",
        )
        .bind(group_id)
        .execute(&self.pool)
        .await?;

        Ok(member)
    }

    pub async fn get_member(
        &self,
        group_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<GroupMember>, AppError> {
        let member = sqlx::query_as::<_, GroupMember>(
            "SELECT * FROM group_members WHERE group_id = $1 AND user_id = $2",
        )
        .bind(group_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(member)
    }

    pub async fn get_members(&self, group_id: Uuid, offset: i32, limit: i32) -> Result<Vec<GroupMember>, AppError> {
        let members = sqlx::query_as::<_, GroupMember>(
            r#"
            SELECT * FROM group_members
            WHERE group_id = $1
            ORDER BY joined_at ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(group_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(members)
    }

    pub async fn remove_member(&self, group_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "DELETE FROM group_members WHERE group_id = $1 AND user_id = $2",
        )
        .bind(group_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        // Update member count
        sqlx::query(
            "UPDATE groups SET member_count = GREATEST(member_count - 1, 0) WHERE id = $1",
        )
        .bind(group_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_member_role(
        &self,
        group_id: Uuid,
        user_id: Uuid,
        role: GroupRole,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE group_members SET role = $1 WHERE group_id = $2 AND user_id = $3",
        )
        .bind(role)
        .bind(group_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Invitation methods
    pub async fn create_invitation(
        &self,
        group_id: Uuid,
        inviter_id: Uuid,
        invitee_id: Uuid,
    ) -> Result<GroupInvitation, AppError> {
        let invitation = sqlx::query_as::<_, GroupInvitation>(
            r#"
            INSERT INTO group_invitations (group_id, inviter_id, invitee_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(group_id)
        .bind(inviter_id)
        .bind(invitee_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(invitation)
    }

    pub async fn get_invitation(&self, invitation_id: Uuid) -> Result<Option<GroupInvitation>, AppError> {
        let invitation = sqlx::query_as::<_, GroupInvitation>(
            "SELECT * FROM group_invitations WHERE id = $1",
        )
        .bind(invitation_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(invitation)
    }

    pub async fn get_pending_invitations_for_user(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<GroupInvitation>, AppError> {
        let invitations = sqlx::query_as::<_, GroupInvitation>(
            r#"
            SELECT * FROM group_invitations
            WHERE invitee_id = $1 AND status = 'pending'
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(invitations)
    }

    pub async fn update_invitation_status(
        &self,
        invitation_id: Uuid,
        status: InvitationStatus,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE group_invitations SET status = $1 WHERE id = $2",
        )
        .bind(status)
        .bind(invitation_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Join request methods
    pub async fn create_join_request(
        &self,
        group_id: Uuid,
        user_id: Uuid,
        message: Option<&str>,
    ) -> Result<GroupJoinRequest, AppError> {
        let request = sqlx::query_as::<_, GroupJoinRequest>(
            r#"
            INSERT INTO group_join_requests (group_id, user_id, message)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(group_id)
        .bind(user_id)
        .bind(message)
        .fetch_one(&self.pool)
        .await?;

        Ok(request)
    }

    pub async fn get_join_request(&self, request_id: Uuid) -> Result<Option<GroupJoinRequest>, AppError> {
        let request = sqlx::query_as::<_, GroupJoinRequest>(
            "SELECT * FROM group_join_requests WHERE id = $1",
        )
        .bind(request_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(request)
    }

    pub async fn get_pending_join_requests_for_group(
        &self,
        group_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<GroupJoinRequest>, AppError> {
        let requests = sqlx::query_as::<_, GroupJoinRequest>(
            r#"
            SELECT * FROM group_join_requests
            WHERE group_id = $1 AND status = 'pending'
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(group_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(requests)
    }

    pub async fn update_join_request_status(
        &self,
        request_id: Uuid,
        status: JoinRequestStatus,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE group_join_requests SET status = $1 WHERE id = $2",
        )
        .bind(status)
        .bind(request_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn has_pending_join_request(&self, group_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM group_join_requests WHERE group_id = $1 AND user_id = $2 AND status = 'pending'",
        )
        .bind(group_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    pub async fn has_pending_invitation(&self, group_id: Uuid, invitee_id: Uuid) -> Result<bool, AppError> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM group_invitations WHERE group_id = $1 AND invitee_id = $2 AND status = 'pending'",
        )
        .bind(group_id)
        .bind(invitee_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}