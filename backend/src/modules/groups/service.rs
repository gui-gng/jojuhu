use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::security::{sanitize_input, validate_input_safety};
use crate::middleware::validation::validate_content_length;
use crate::modules::users::repository::UserRepository;

use super::models::{
    CreateGroupRequest, GroupMemberResponse, GroupResponse, GroupRole, InvitationResponse,
    InvitationStatus, JoinRequestResponse, JoinRequestStatus, UpdateGroupRequest,
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

    // Invitation methods
    pub async fn invite_user(
        &self,
        group_id: Uuid,
        inviter_id: Uuid,
        invitee_id: Uuid,
        user_repository: &UserRepository,
    ) -> Result<InvitationResponse, AppError> {
        // Check if inviter is admin or moderator
        let inviter_membership = self
            .repository
            .get_member(group_id, inviter_id)
            .await?
            .ok_or_else(|| AppError::AuthorizationError("Not a member of this group".to_string()))?;

        match inviter_membership.role {
            GroupRole::Admin | GroupRole::Moderator => (),
            GroupRole::Member => {
                return Err(AppError::AuthorizationError(
                    "Only admins and moderators can invite users".to_string(),
                ));
            }
        }

        // Check if group exists
        let group = self
            .repository
            .get_group_by_id(group_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Group not found".to_string()))?;

        // Check if invitee exists
        user_repository
            .get_user_profile(invitee_id, None)
            .await?
            .ok_or_else(|| AppError::NotFoundError("User not found".to_string()))?;

        // Check if already a member
        if self.repository.get_member(group_id, invitee_id).await?.is_some() {
            return Err(AppError::ValidationError("User is already a member of this group".to_string()));
        }

        // Check for pending invitation
        if self.repository.has_pending_invitation(group_id, invitee_id).await? {
            return Err(AppError::ValidationError("User already has a pending invitation".to_string()));
        }

        // Check for pending join request
        if self.repository.has_pending_join_request(group_id, invitee_id).await? {
            return Err(AppError::ValidationError("User already has a pending join request".to_string()));
        }

        let invitation = self.repository.create_invitation(group_id, inviter_id, invitee_id).await?;

        let inviter = user_repository
            .get_user_profile(inviter_id, None)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Inviter not found".to_string()))?;

        Ok(InvitationResponse {
            id: invitation.id,
            group_id: invitation.group_id,
            group_name: group.name,
            inviter_id: invitation.inviter_id,
            inviter_username: inviter.username,
            status: invitation.status,
            created_at: invitation.created_at,
        })
    }

    pub async fn get_my_invitations(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
        user_repository: &UserRepository,
    ) -> Result<Vec<InvitationResponse>, AppError> {
        let invitations = self.repository.get_pending_invitations_for_user(user_id, offset, limit).await?;

        let mut responses = Vec::new();
        for invitation in invitations {
            if let Ok(Some(group)) = self.repository.get_group_by_id(invitation.group_id).await {
                if let Ok(Some(inviter)) = user_repository.get_user_profile(invitation.inviter_id, None).await {
                    responses.push(InvitationResponse {
                        id: invitation.id,
                        group_id: invitation.group_id,
                        group_name: group.name,
                        inviter_id: invitation.inviter_id,
                        inviter_username: inviter.username,
                        status: invitation.status,
                        created_at: invitation.created_at,
                    });
                }
            }
        }

        Ok(responses)
    }

    pub async fn accept_invitation(
        &self,
        invitation_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let invitation = self
            .repository
            .get_invitation(invitation_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Invitation not found".to_string()))?;

        if invitation.invitee_id != user_id {
            return Err(AppError::AuthorizationError("You cannot accept this invitation".to_string()));
        }

        if !matches!(invitation.status, InvitationStatus::Pending) {
            return Err(AppError::ValidationError("Invitation is not pending".to_string()));
        }

        // Add user as member
        self.repository
            .add_member(invitation.group_id, user_id, GroupRole::Member)
            .await?;

        // Update invitation status
        self.repository
            .update_invitation_status(invitation_id, InvitationStatus::Accepted)
            .await?;

        Ok(())
    }

    pub async fn reject_invitation(
        &self,
        invitation_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let invitation = self
            .repository
            .get_invitation(invitation_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Invitation not found".to_string()))?;

        if invitation.invitee_id != user_id {
            return Err(AppError::AuthorizationError("You cannot reject this invitation".to_string()));
        }

        if !matches!(invitation.status, InvitationStatus::Pending) {
            return Err(AppError::ValidationError("Invitation is not pending".to_string()));
        }

        self.repository
            .update_invitation_status(invitation_id, InvitationStatus::Rejected)
            .await?;

        Ok(())
    }

    // Join request methods
    pub async fn request_join_group(
        &self,
        group_id: Uuid,
        user_id: Uuid,
        message: Option<String>,
    ) -> Result<JoinRequestResponse, AppError> {
        let group = self
            .repository
            .get_group_by_id(group_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Group not found".to_string()))?;

        if !group.is_private {
            return Err(AppError::ValidationError("Public groups don't require join requests".to_string()));
        }

        // Check if already a member
        if self.repository.get_member(group_id, user_id).await?.is_some() {
            return Err(AppError::ValidationError("Already a member of this group".to_string()));
        }

        // Check for pending request
        if self.repository.has_pending_join_request(group_id, user_id).await? {
            return Err(AppError::ValidationError("You already have a pending join request".to_string()));
        }

        // Check for pending invitation
        if self.repository.has_pending_invitation(group_id, user_id).await? {
            return Err(AppError::ValidationError("You already have a pending invitation".to_string()));
        }

        let sanitized_message = message.map(|m| {
            validate_input_safety(&m).ok();
            sanitize_input(&m)
        });

        let request = self
            .repository
            .create_join_request(group_id, user_id, sanitized_message.as_deref())
            .await?;

        Ok(JoinRequestResponse {
            id: request.id,
            group_id: request.group_id,
            group_name: group.name,
            user_id: request.user_id,
            username: String::new(),
            message: request.message,
            status: request.status,
            created_at: request.created_at,
        })
    }

    pub async fn get_join_requests(
        &self,
        group_id: Uuid,
        user_id: Uuid,
        offset: i32,
        limit: i32,
        user_repository: &UserRepository,
    ) -> Result<Vec<JoinRequestResponse>, AppError> {
        // Check if user is admin or moderator
        let membership = self
            .repository
            .get_member(group_id, user_id)
            .await?
            .ok_or_else(|| AppError::AuthorizationError("Not a member of this group".to_string()))?;

        match membership.role {
            GroupRole::Admin | GroupRole::Moderator => (),
            GroupRole::Member => {
                return Err(AppError::AuthorizationError(
                    "Only admins and moderators can view join requests".to_string(),
                ));
            }
        }

        let requests = self.repository.get_pending_join_requests_for_group(group_id, offset, limit).await?;

        let mut responses = Vec::new();
        for request in requests {
            if let Ok(Some(user)) = user_repository.get_user_profile(request.user_id, None).await {
                responses.push(JoinRequestResponse {
                    id: request.id,
                    group_id: request.group_id,
                    group_name: String::new(),
                    user_id: request.user_id,
                    username: user.username,
                    message: request.message,
                    status: request.status,
                    created_at: request.created_at,
                });
            }
        }

        Ok(responses)
    }

    pub async fn approve_join_request(
        &self,
        request_id: Uuid,
        approver_id: Uuid,
    ) -> Result<(), AppError> {
        let request = self
            .repository
            .get_join_request(request_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Join request not found".to_string()))?;

        // Check if approver is admin or moderator
        let membership = self
            .repository
            .get_member(request.group_id, approver_id)
            .await?
            .ok_or_else(|| AppError::AuthorizationError("Not a member of this group".to_string()))?;

        match membership.role {
            GroupRole::Admin | GroupRole::Moderator => (),
            GroupRole::Member => {
                return Err(AppError::AuthorizationError(
                    "Only admins and moderators can approve join requests".to_string(),
                ));
            }
        }

        if !matches!(request.status, JoinRequestStatus::Pending) {
            return Err(AppError::ValidationError("Join request is not pending".to_string()));
        }

        // Add user as member
        self.repository
            .add_member(request.group_id, request.user_id, GroupRole::Member)
            .await?;

        // Update request status
        self.repository
            .update_join_request_status(request_id, JoinRequestStatus::Approved)
            .await?;

        Ok(())
    }

    pub async fn reject_join_request(
        &self,
        request_id: Uuid,
        rejector_id: Uuid,
    ) -> Result<(), AppError> {
        let request = self
            .repository
            .get_join_request(request_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Join request not found".to_string()))?;

        // Check if rejector is admin or moderator
        let membership = self
            .repository
            .get_member(request.group_id, rejector_id)
            .await?
            .ok_or_else(|| AppError::AuthorizationError("Not a member of this group".to_string()))?;

        match membership.role {
            GroupRole::Admin | GroupRole::Moderator => (),
            GroupRole::Member => {
                return Err(AppError::AuthorizationError(
                    "Only admins and moderators can reject join requests".to_string(),
                ));
            }
        }

        if !matches!(request.status, JoinRequestStatus::Pending) {
            return Err(AppError::ValidationError("Join request is not pending".to_string()));
        }

        self.repository
            .update_join_request_status(request_id, JoinRequestStatus::Rejected)
            .await?;

        Ok(())
    }

    // Role management methods
    pub async fn update_member_role(
        &self,
        group_id: Uuid,
        user_id: Uuid,
        target_user_id: Uuid,
        new_role: GroupRole,
    ) -> Result<(), AppError> {
        // Check if user is admin
        let membership = self
            .repository
            .get_member(group_id, user_id)
            .await?
            .ok_or_else(|| AppError::AuthorizationError("Not a member of this group".to_string()))?;

        match membership.role {
            GroupRole::Admin => (),
            _ => return Err(AppError::AuthorizationError("Only admins can change member roles".to_string())),
        }

        // Check if target user is member
        let target_membership = self
            .repository
            .get_member(group_id, target_user_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("User is not a member of this group".to_string()))?;

        // Prevent changing role of last admin
        if matches!(target_membership.role, GroupRole::Admin) && matches!(new_role, GroupRole::Moderator | GroupRole::Member) {
            let members = self.repository.get_members(group_id, 0, 100).await?;
            let admin_count = members.iter().filter(|m| matches!(m.role, GroupRole::Admin)).count();
            
            if admin_count == 1 {
                return Err(AppError::ValidationError("Cannot remove the last admin".to_string()));
            }
        }

        self.repository.update_member_role(group_id, target_user_id, new_role).await?;

        Ok(())
    }
}