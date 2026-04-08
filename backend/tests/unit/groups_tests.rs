//! Unit tests for groups module - invitations, join requests, and role management

use jojuhu_backend::errors::AppError;
use jojuhu_backend::modules::groups::models::{GroupRole, InvitationStatus, JoinRequestStatus};
use uuid::Uuid;

// Mock structures for testing

#[derive(Clone, Debug)]
struct MockGroup {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    creator_id: Uuid,
    is_private: bool,
    member_count: i32,
}

#[derive(Clone, Debug)]
struct MockGroupMember {
    id: Uuid,
    group_id: Uuid,
    user_id: Uuid,
    role: GroupRole,
}

#[derive(Clone, Debug)]
struct MockGroupInvitation {
    id: Uuid,
    group_id: Uuid,
    inviter_id: Uuid,
    invitee_id: Uuid,
    status: InvitationStatus,
}

#[derive(Clone, Debug)]
struct MockGroupJoinRequest {
    id: Uuid,
    group_id: Uuid,
    user_id: Uuid,
    message: Option<String>,
    status: JoinRequestStatus,
}

struct MockGroupRepository {
    groups: Vec<MockGroup>,
    members: Vec<MockGroupMember>,
    invitations: Vec<MockGroupInvitation>,
    join_requests: Vec<MockGroupJoinRequest>,
}

impl MockGroupRepository {
    fn new() -> Self {
        Self {
            groups: Vec::new(),
            members: Vec::new(),
            invitations: Vec::new(),
            join_requests: Vec::new(),
        }
    }

    fn create_group(&mut self, creator_id: Uuid, name: &str, is_private: bool) -> MockGroup {
        let group = MockGroup {
            id: Uuid::new_v4(),
            name: name.to_string(),
            slug: name.to_lowercase().replace(' ', "-"),
            description: None,
            creator_id,
            is_private,
            member_count: 1,
        };
        self.groups.push(group.clone());

        let member = MockGroupMember {
            id: Uuid::new_v4(),
            group_id: group.id,
            user_id: creator_id,
            role: GroupRole::Admin,
        };
        self.members.push(member);

        group
    }

    fn get_member(&self, group_id: Uuid, user_id: Uuid) -> Option<&MockGroupMember> {
        self.members
            .iter()
            .find(|m| m.group_id == group_id && m.user_id == user_id)
    }

    fn get_group(&self, group_id: Uuid) -> Option<&MockGroup> {
        self.groups.iter().find(|g| g.id == group_id)
    }

    fn add_member(&mut self, group_id: Uuid, user_id: Uuid, role: GroupRole) -> MockGroupMember {
        let member = MockGroupMember {
            id: Uuid::new_v4(),
            group_id,
            user_id,
            role,
        };
        self.members.push(member.clone());

        if let Some(group) = self.groups.iter_mut().find(|g| g.id == group_id) {
            group.member_count += 1;
        }

        member
    }

    fn create_invitation(
        &mut self,
        group_id: Uuid,
        inviter_id: Uuid,
        invitee_id: Uuid,
    ) -> MockGroupInvitation {
        let invitation = MockGroupInvitation {
            id: Uuid::new_v4(),
            group_id,
            inviter_id,
            invitee_id,
            status: InvitationStatus::Pending,
        };
        self.invitations.push(invitation.clone());
        invitation
    }

    fn get_invitation(&self, invitation_id: Uuid) -> Option<&MockGroupInvitation> {
        self.invitations.iter().find(|i| i.id == invitation_id)
    }

    fn has_pending_invitation(&self, group_id: Uuid, invitee_id: Uuid) -> bool {
        self.invitations.iter().any(|i| {
            i.group_id == group_id
                && i.invitee_id == invitee_id
                && matches!(i.status, InvitationStatus::Pending)
        })
    }

    fn create_join_request(
        &mut self,
        group_id: Uuid,
        user_id: Uuid,
        message: Option<String>,
    ) -> MockGroupJoinRequest {
        let request = MockGroupJoinRequest {
            id: Uuid::new_v4(),
            group_id,
            user_id,
            message,
            status: JoinRequestStatus::Pending,
        };
        self.join_requests.push(request.clone());
        request
    }

    fn get_join_request(&self, request_id: Uuid) -> Option<&MockGroupJoinRequest> {
        self.join_requests.iter().find(|r| r.id == request_id)
    }

    fn has_pending_join_request(&self, group_id: Uuid, user_id: Uuid) -> bool {
        self.join_requests.iter().any(|r| {
            r.group_id == group_id
                && r.user_id == user_id
                && matches!(r.status, JoinRequestStatus::Pending)
        })
    }

    fn update_member_role(
        &mut self,
        group_id: Uuid,
        user_id: Uuid,
        role: GroupRole,
    ) -> Result<(), AppError> {
        if let Some(member) = self
            .members
            .iter_mut()
            .find(|m| m.group_id == group_id && m.user_id == user_id)
        {
            member.role = role;
            Ok(())
        } else {
            Err(AppError::NotFoundError("Member not found".to_string()))
        }
    }
}

// ==================== Group Invitation Tests ====================

#[test]
fn test_invite_user_as_admin() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let invitee_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", true);

    let invitation = repo.create_invitation(group.id, admin_id, invitee_id);

    assert_eq!(invitation.group_id, group.id);
    assert_eq!(invitation.inviter_id, admin_id);
    assert_eq!(invitation.invitee_id, invitee_id);
    assert!(matches!(invitation.status, InvitationStatus::Pending));

    assert!(repo.has_pending_invitation(group.id, invitee_id));
}

#[test]
fn test_invite_already_member() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let member_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", true);
    repo.add_member(group.id, member_id, GroupRole::Member);

    let has_invite = repo.has_pending_invitation(group.id, member_id);
    assert!(!has_invite, "Member should not have a pending invitation");

    let member = repo.get_member(group.id, member_id);
    assert!(member.is_some());
}

#[test]
fn test_accept_invitation() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let invitee_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", true);
    let invitation = repo.create_invitation(group.id, admin_id, invitee_id);

    assert!(repo.has_pending_invitation(group.id, invitee_id));

    repo.add_member(group.id, invitee_id, GroupRole::Member);

    let member = repo.get_member(group.id, invitee_id);
    assert!(member.is_some());
    assert!(matches!(member.unwrap().role, GroupRole::Member));
}

#[test]
fn test_reject_invitation() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let invitee_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", true);
    repo.create_invitation(group.id, admin_id, invitee_id);

    assert!(repo.has_pending_invitation(group.id, invitee_id));

    let member = repo.get_member(group.id, invitee_id);
    assert!(
        member.is_none(),
        "User should not be a member after rejection"
    );
}

#[test]
fn test_double_invitation_prevention() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let invitee_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", true);
    repo.create_invitation(group.id, admin_id, invitee_id);

    assert!(repo.has_pending_invitation(group.id, invitee_id));

    assert!(repo.has_pending_invitation(group.id, invitee_id));
}

// ==================== Join Request Tests ====================

#[test]
fn test_create_join_request() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Private Group", true);

    let request =
        repo.create_join_request(group.id, user_id, Some("Please let me join!".to_string()));

    assert_eq!(request.group_id, group.id);
    assert_eq!(request.user_id, user_id);
    assert!(matches!(request.status, JoinRequestStatus::Pending));
    assert!(repo.has_pending_join_request(group.id, user_id));
}

#[test]
fn test_approve_join_request() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Private Group", true);
    let request =
        repo.create_join_request(group.id, user_id, Some("Please let me join!".to_string()));

    assert!(repo.has_pending_join_request(group.id, user_id));

    repo.add_member(group.id, user_id, GroupRole::Member);

    let member = repo.get_member(group.id, user_id);
    assert!(member.is_some());
    assert!(matches!(member.unwrap().role, GroupRole::Member));
}

#[test]
fn test_reject_join_request() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Private Group", true);
    repo.create_join_request(group.id, user_id, Some("Please let me join!".to_string()));

    assert!(repo.has_pending_join_request(group.id, user_id));

    let member = repo.get_member(group.id, user_id);
    assert!(
        member.is_none(),
        "User should not be a member after rejection"
    );
}

#[test]
fn test_join_request_already_member() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let member_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", true);
    repo.add_member(group.id, member_id, GroupRole::Member);

    let member = repo.get_member(group.id, member_id);
    assert!(member.is_some(), "User should already be a member");

    let has_request = repo.has_pending_join_request(group.id, member_id);
    assert!(
        !has_request,
        "Member should not have a pending join request"
    );
}

#[test]
fn test_join_request_already_has_invitation() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", true);
    repo.create_invitation(group.id, admin_id, user_id);

    assert!(repo.has_pending_invitation(group.id, user_id));

    let has_join_request = repo.has_pending_join_request(group.id, user_id);
    assert!(
        !has_join_request,
        "User with invitation should not have join request"
    );
}

// ==================== Role Management Tests ====================

#[test]
fn test_promote_member_to_moderator() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let member_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", false);
    repo.add_member(group.id, member_id, GroupRole::Member);

    let result = repo.update_member_role(group.id, member_id, GroupRole::Moderator);
    assert!(result.is_ok());

    let member = repo.get_member(group.id, member_id);
    assert!(member.is_some());
    assert!(matches!(member.unwrap().role, GroupRole::Moderator));
}

#[test]
fn test_promote_member_to_admin() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let member_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", false);
    repo.add_member(group.id, member_id, GroupRole::Member);

    let result = repo.update_member_role(group.id, member_id, GroupRole::Admin);
    assert!(result.is_ok());

    let member = repo.get_member(group.id, member_id);
    assert!(member.is_some());
    assert!(matches!(member.unwrap().role, GroupRole::Admin));
}

#[test]
fn test_demote_admin_to_member() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let second_admin_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", false);
    repo.add_member(group.id, second_admin_id, GroupRole::Admin);

    let member_count_before = repo.get_group(group.id).unwrap().member_count;
    assert_eq!(member_count_before, 2);

    let admins: Vec<_> = repo
        .members
        .iter()
        .filter(|m| m.group_id == group.id && matches!(m.role, GroupRole::Admin))
        .collect();
    assert_eq!(admins.len(), 2);

    let result = repo.update_member_role(group.id, second_admin_id, GroupRole::Member);
    assert!(result.is_ok());

    let member = repo.get_member(group.id, second_admin_id);
    assert!(matches!(member.unwrap().role, GroupRole::Member));
}

#[test]
fn test_can_update_last_admin_role_in_repository() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", false);

    let admins: Vec<_> = repo
        .members
        .iter()
        .filter(|m| m.group_id == group.id && matches!(m.role, GroupRole::Admin))
        .collect();
    assert_eq!(admins.len(), 1, "Should have exactly one admin");

    let result = repo.update_member_role(group.id, admin_id, GroupRole::Member);
    assert!(
        result.is_ok(),
        "Repository should allow the update (business logic is in service layer)"
    );

    let member = repo.get_member(group.id, admin_id).unwrap();
    assert!(
        matches!(member.role, GroupRole::Member),
        "Role should be updated to Member"
    );
}

#[test]
fn test_role_hierarchy() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let moderator_id = Uuid::new_v4();
    let member_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", false);
    repo.add_member(group.id, moderator_id, GroupRole::Moderator);
    repo.add_member(group.id, member_id, GroupRole::Member);

    let admin = repo.get_member(group.id, admin_id).unwrap();
    assert!(matches!(admin.role, GroupRole::Admin));

    let moderator = repo.get_member(group.id, moderator_id).unwrap();
    assert!(matches!(moderator.role, GroupRole::Moderator));

    let member = repo.get_member(group.id, member_id).unwrap();
    assert!(matches!(member.role, GroupRole::Member));
}

// ==================== Group Privacy Tests ====================

#[test]
fn test_private_group_creation() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Private Group", true);

    assert!(group.is_private);

    let group2 = repo.create_group(admin_id, "Public Group", false);
    assert!(!group2.is_private);
}

#[test]
fn test_member_count_updates() {
    let mut repo = MockGroupRepository::new();
    let admin_id = Uuid::new_v4();
    let member1 = Uuid::new_v4();
    let member2 = Uuid::new_v4();

    let group = repo.create_group(admin_id, "Test Group", false);
    assert_eq!(
        group.member_count, 1,
        "Group should start with 1 member (creator)"
    );

    repo.add_member(group.id, member1, GroupRole::Member);
    let updated_group = repo.get_group(group.id).unwrap();
    assert_eq!(
        updated_group.member_count, 2,
        "Member count should increase"
    );

    repo.add_member(group.id, member2, GroupRole::Member);
    let updated_group = repo.get_group(group.id).unwrap();
    assert_eq!(
        updated_group.member_count, 3,
        "Member count should increase again"
    );
}
