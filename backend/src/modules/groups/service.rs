use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::security::{sanitize_input, validate_input_safety};
use crate::middleware::validation::validate_content_length;
use crate::modules::users::repository::UserRepository;

use super::models::{
    CreateGroupRequest, GroupMemberResponse, GroupResponse, GroupRole, UpdateGroupRequest,
};
use super::repository::GroupRepository;

const MIN_NAME_LENGTH: usize = 2;
const MAX_NAME_LENGTH: usize = 100;
const MAX_DESCRIPTION_LENGTH: usize = 500;

pub struct GroupService {
    repository: GroupRepository,
}

impl GroupService {
    pub fn new(repository: GroupRepository) -> Self {
        Self { repository }
    }

    fn generate_slug(name: &str) -> String {
        name.to_lowercase()
            .replace(' ', "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
    }

    pub async fn create_group(
        &self,
        creator_id: Uuid,
        mut request: CreateGroupRequest,
    ) -> Result<GroupResponse, AppError> {
        // Validate name
        validate_content_length(
            &request.name,
            MIN_NAME_LENGTH,
            MAX_NAME_LENGTH,
            "Group name",
        )?;
        validate_input_safety(&request.name).map_err(AppError::ValidationError)?;
        request.name = sanitize_input(&request.name);

        // Sanitize description
        if let Some(ref mut desc) = request.description {
            if !desc.is_empty() {
                validate_content_length(desc, 0, MAX_DESCRIPTION_LENGTH, "Group description")?;
                validate_input_safety(desc).map_err(AppError::ValidationError)?;
                *desc = sanitize_input(desc);
            }
        }

        let slug = Self::generate_slug(&request.name);
        let is_private = request.is_private.unwrap_or(true);

        let group = self
            .repository
            .create_group(
                &request.name,
                &slug,
                request.description.as_deref(),
                creator_id,
                is_private,
            )
            .await?;

        Ok(GroupResponse {
            id: group.id,
            name: group.name,
            slug: group.slug,
            description: group.description,
            creator_id: group.creator_id,
            is_private: group.is_private,
            member_count: group.member_count,
            is_member: true,
            role: Some(GroupRole::Admin),
            created_at: group.created_at,
        })
    }

    pub async fn get_group(&self, group_id: Uuid, user_id: Option<Uuid>) -> Result<GroupResponse, AppError> {
        let group = self
            .repository
            .get_group_by_id(group_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Group not found".to_string()))?;

        // Check membership
        let (is_member, role) = if let Some(uid) = user_id {
            match self.repository.get_member(group_id, uid).await? {
                Some(m) => (true, Some(m.role)),
                None => (false, None),
            }
        } else {
            (false, None)
        };

        Ok(GroupResponse {
            id: group.id,
            name: group.name,
            slug: group.slug,
            description: group.description,
            creator_id: group.creator_id,
            is_private: group.is_private,
            member_count: group.member_count,
            is_member,
            role,
            created_at: group.created_at,
        })
    }

    pub async fn list_groups(
        &self,
        offset: i32,
        limit: i32,
        user_id: Option<Uuid>,
    ) -> Result<Vec<GroupResponse>, AppError> {
        let groups = self.repository.list_groups(offset, limit, user_id).await?;

        let mut responses = Vec::new();
        for group in groups {
            let (is_member, role) = if let Some(uid) = user_id {
                match self.repository.get_member(group.id, uid).await? {
                    Some(m) => (true, Some(m.role)),
                    None => (false, None),
                }
            } else {
                (false, None)
            };

            responses.push(GroupResponse {
                id: group.id,
                name: group.name,
                slug: group.slug,
                description: group.description,
                creator_id: group.creator_id,
                is_private: group.is_private,
                member_count: group.member_count,
                is_member,
                role,
                created_at: group.created_at,
            });
        }

        Ok(responses)
    }

    pub async fn update_group(
        &self,
        group_id: Uuid,
        user_id: Uuid,
        mut request: UpdateGroupRequest,
    ) -> Result<GroupResponse, AppError> {
        // Check if user is admin
        let member = self
            .repository
            .get_member(group_id, user_id)
            .await?
            .ok_or_else(|| AppError::AuthorizationError("Not a member of this group".to_string()))?;

        match member.role {
            GroupRole::Admin => (),
            _ => return Err(AppError::AuthorizationError("Only admins can update groups".to_string())),
        }

        // Validate and sanitize
        if let Some(ref mut name) = request.name {
            validate_content_length(name, MIN_NAME_LENGTH, MAX_NAME_LENGTH, "Group name")?;
            validate_input_safety(name).map_err(AppError::ValidationError)?;
            *name = sanitize_input(name);
        }

        if let Some(ref mut desc) = request.description {
            if !desc.is_empty() {
                validate_content_length(desc, 0, MAX_DESCRIPTION_LENGTH, "Group description")?;
                validate_input_safety(desc).map_err(AppError::ValidationError)?;
                *desc = sanitize_input(desc);
            }
        }

        let group = self
            .repository
            .update_group(
                group_id,
                request.name.as_deref(),
                request.description.as_deref(),
                request.is_private,
            )
            .await?;

        Ok(GroupResponse {
            id: group.id,
            name: group.name,
            slug: group.slug,
            description: group.description,
            creator_id: group.creator_id,
            is_private: group.is_private,
            member_count: group.member_count,
            is_member: true,
            role: Some(GroupRole::Admin),
            created_at: group.created_at,
        })
    }

    pub async fn delete_group(&self, group_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        // Check if user is admin
        let member = self
            .repository
            .get_member(group_id, user_id)
            .await?
            .ok_or_else(|| AppError::AuthorizationError("Not a member of this group".to_string()))?;

        match member.role {
            GroupRole::Admin => (),
            _ => return Err(AppError::AuthorizationError("Only admins can delete groups".to_string())),
        }

        self.repository.delete_group(group_id).await
    }

    pub async fn join_group(&self, group_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let group = self
            .repository
            .get_group_by_id(group_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Group not found".to_string()))?;

        // Check if already a member
        if self.repository.get_member(group_id, user_id).await?.is_some() {
            return Err(AppError::ValidationError("Already a member of this group".to_string()));
        }

        if group.is_private {
            // For private groups, create a join request instead
            // For now, we'll just allow joining with a simple flow
            // In production, you'd check for invitations
            return Err(AppError::ValidationError("Private groups require invitation to join".to_string()));
        }

        self.repository
            .add_member(group_id, user_id, GroupRole::Member)
            .await?;

        Ok(())
    }

    pub async fn leave_group(&self, group_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        // Check if member
        if self.repository.get_member(group_id, user_id).await?.is_none() {
            return Err(AppError::ValidationError("Not a member of this group".to_string()));
        }

        // Check if last admin
        let members = self.repository.get_members(group_id, 0, 100).await?;
        let admin_count = members.iter().filter(|m| matches!(m.role, GroupRole::Admin)).count();
        
        let is_admin = self
            .repository
            .get_member(group_id, user_id)
            .await?
            .map(|m| matches!(m.role, GroupRole::Admin))
            .unwrap_or(false);

        if is_admin && admin_count == 1 {
            return Err(AppError::ValidationError("Cannot leave group as the last admin".to_string()));
        }

        self.repository.remove_member(group_id, user_id).await
    }

    pub async fn get_members(
        &self,
        group_id: Uuid,
        offset: i32,
        limit: i32,
        user_repository: &UserRepository,
    ) -> Result<Vec<GroupMemberResponse>, AppError> {
        // Check if user is a member (for private groups)
        let members = self.repository.get_members(group_id, offset, limit).await?;

        let mut responses = Vec::new();
        for member in members {
            if let Ok(Some(user)) = user_repository.get_user_profile(member.user_id, None).await {
                responses.push(GroupMemberResponse {
                    id: member.id,
                    user_id: member.user_id,
                    username: user.username,
                    display_name: user.display_name,
                    avatar_url: user.avatar_url,
                    role: member.role,
                    joined_at: member.joined_at,
                });
            }
        }

        Ok(responses)
    }
}