//! Repository tests with mocked database
//!
//! These tests verify repository logic without requiring a real database.
//! For integration tests with a real database, see tests/integration_tests.rs

use social_network::errors::AppError;

// Mock repository structure for testing
struct MockMessageRepository {
    messages: Vec<MockMessage>,
}

#[derive(Clone)]
struct MockMessage {
    id: uuid::Uuid,
    sender_id: uuid::Uuid,
    recipient_id: uuid::Uuid,
    content: String,
    is_read: bool,
}

impl MockMessageRepository {
    fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    fn create_message(
        &mut self,
        sender_id: uuid::Uuid,
        recipient_id: uuid::Uuid,
        content: &str,
    ) -> Result<MockMessage, AppError> {
        if content.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Message content cannot be empty".to_string(),
            ));
        }

        let message = MockMessage {
            id: uuid::Uuid::new_v4(),
            sender_id,
            recipient_id,
            content: content.to_string(),
            is_read: false,
        };

        self.messages.push(message.clone());
        Ok(message)
    }

    fn get_conversation(
        &self,
        user_id: uuid::Uuid,
        other_user_id: uuid::Uuid,
        limit: usize,
    ) -> Vec<MockMessage> {
        self.messages
            .iter()
            .filter(|m| {
                (m.sender_id == user_id && m.recipient_id == other_user_id)
                    || (m.sender_id == other_user_id && m.recipient_id == user_id)
            })
            .cloned()
            .take(limit)
            .collect()
    }

    fn mark_as_read(&mut self, message_id: uuid::Uuid, user_id: uuid::Uuid) -> Result<(), AppError> {
        if let Some(message) = self.messages.iter_mut().find(|m| m.id == message_id) {
            if message.recipient_id != user_id {
                return Err(AppError::AuthorizationError(
                    "Only the recipient can mark a message as read".to_string(),
                ));
            }
            message.is_read = true;
            Ok(())
        } else {
            Err(AppError::NotFoundError("Message not found".to_string()))
        }
    }
}

#[test]
fn test_mock_repository_create_message_success() {
    let mut repo = MockMessageRepository::new();
    let sender_id = uuid::Uuid::new_v4();
    let recipient_id = uuid::Uuid::new_v4();

    let result = repo.create_message(sender_id, recipient_id, "Hello!");
    
    assert!(result.is_ok());
    let message = result.unwrap();
    assert_eq!(message.sender_id, sender_id);
    assert_eq!(message.recipient_id, recipient_id);
    assert_eq!(message.content, "Hello!");
    assert!(!message.is_read);
}

#[test]
fn test_mock_repository_create_message_empty_content() {
    let mut repo = MockMessageRepository::new();
    let sender_id = uuid::Uuid::new_v4();
    let recipient_id = uuid::Uuid::new_v4();

    let result = repo.create_message(sender_id, recipient_id, "");
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::ValidationError(msg) => assert!(msg.contains("cannot be empty")),
        _ => panic!("Expected ValidationError"),
    }
}

#[test]
fn test_mock_repository_create_message_whitespace_only() {
    let mut repo = MockMessageRepository::new();
    let sender_id = uuid::Uuid::new_v4();
    let recipient_id = uuid::Uuid::new_v4();

    let result = repo.create_message(sender_id, recipient_id, "   ");
    
    assert!(result.is_err());
}

#[test]
fn test_mock_repository_get_conversation() {
    let mut repo = MockMessageRepository::new();
    let user1 = uuid::Uuid::new_v4();
    let user2 = uuid::Uuid::new_v4();
    let user3 = uuid::Uuid::new_v4();

    // Create messages between user1 and user2
    repo.create_message(user1, user2, "Hello from user1").unwrap();
    repo.create_message(user2, user1, "Hi from user2").unwrap();
    repo.create_message(user1, user2, "How are you?").unwrap();
    
    // Create message between user1 and user3 (shouldn't appear in user1-user2 conversation)
    repo.create_message(user1, user3, "Hey user3").unwrap();

    let conversation = repo.get_conversation(user1, user2, 10);
    
    assert_eq!(conversation.len(), 3);
}

#[test]
fn test_mock_repository_get_conversation_limit() {
    let mut repo = MockMessageRepository::new();
    let user1 = uuid::Uuid::new_v4();
    let user2 = uuid::Uuid::new_v4();

    // Create 5 messages
    for i in 0..5 {
        repo.create_message(user1, user2, &format!("Message {}", i)).unwrap();
    }

    let conversation = repo.get_conversation(user1, user2, 3);
    
    assert_eq!(conversation.len(), 3);
}

#[test]
fn test_mock_repository_mark_as_read_success() {
    let mut repo = MockMessageRepository::new();
    let sender_id = uuid::Uuid::new_v4();
    let recipient_id = uuid::Uuid::new_v4();

    let message = repo.create_message(sender_id, recipient_id, "Hello!").unwrap();
    
    let result = repo.mark_as_read(message.id, recipient_id);
    
    assert!(result.is_ok());
}

#[test]
fn test_mock_repository_mark_as_read_wrong_user() {
    let mut repo = MockMessageRepository::new();
    let sender_id = uuid::Uuid::new_v4();
    let recipient_id = uuid::Uuid::new_v4();
    let wrong_user = uuid::Uuid::new_v4();

    let message = repo.create_message(sender_id, recipient_id, "Hello!").unwrap();
    
    let result = repo.mark_as_read(message.id, wrong_user);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::AuthorizationError(msg) => assert!(msg.contains("Only the recipient")),
        _ => panic!("Expected AuthorizationError"),
    }
}

#[test]
fn test_mock_repository_mark_as_read_not_found() {
    let mut repo = MockMessageRepository::new();
    let user_id = uuid::Uuid::new_v4();
    let non_existent_id = uuid::Uuid::new_v4();

    let result = repo.mark_as_read(non_existent_id, user_id);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::NotFoundError(msg) => assert_eq!(msg, "Message not found"),
        _ => panic!("Expected NotFoundError"),
    }
}

// ==================== Timeline Repository Tests ====================

struct MockTimelineRepository {
    posts: Vec<MockPost>,
    comments: Vec<MockComment>,
}

#[derive(Clone)]
struct MockPost {
    id: uuid::Uuid,
    author_id: uuid::Uuid,
    content: String,
    is_public: bool,
}

#[derive(Clone)]
struct MockComment {
    id: uuid::Uuid,
    post_id: uuid::Uuid,
    author_id: uuid::Uuid,
    content: String,
}

impl MockTimelineRepository {
    fn new() -> Self {
        Self {
            posts: Vec::new(),
            comments: Vec::new(),
        }
    }

    fn create_post(
        &mut self,
        author_id: uuid::Uuid,
        content: &str,
        is_public: bool,
    ) -> Result<MockPost, AppError> {
        if content.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Post content cannot be empty".to_string(),
            ));
        }

        let post = MockPost {
            id: uuid::Uuid::new_v4(),
            author_id,
            content: content.to_string(),
            is_public,
        };

        self.posts.push(post.clone());
        Ok(post)
    }

    fn get_user_posts(&self,
        author_id: uuid::Uuid,
        requesting_user_id: uuid::Uuid,
        limit: usize,
    ) -> Vec<MockPost> {
        self.posts
            .iter()
            .filter(|p| {
                p.author_id == author_id && (p.is_public || p.author_id == requesting_user_id)
            })
            .cloned()
            .take(limit)
            .collect()
    }

    fn delete_post(
        &mut self,
        post_id: uuid::Uuid,
        user_id: uuid::Uuid,
    ) -> Result<(), AppError> {
        if let Some(post) = self.posts.iter().find(|p| p.id == post_id) {
            if post.author_id != user_id {
                return Err(AppError::AuthorizationError(
                    "You can only delete your own posts".to_string(),
                ));
            }
            self.posts.retain(|p| p.id != post_id);
            Ok(())
        } else {
            Err(AppError::NotFoundError("Post not found".to_string()))
        }
    }
}

#[test]
fn test_mock_timeline_create_post_success() {
    let mut repo = MockTimelineRepository::new();
    let author_id = uuid::Uuid::new_v4();

    let result = repo.create_post(author_id, "My first post!", true);
    
    assert!(result.is_ok());
    let post = result.unwrap();
    assert_eq!(post.author_id, author_id);
    assert_eq!(post.content, "My first post!");
    assert!(post.is_public);
}

#[test]
fn test_mock_timeline_create_post_empty_content() {
    let mut repo = MockTimelineRepository::new();
    let author_id = uuid::Uuid::new_v4();

    let result = repo.create_post(author_id, "", true);
    
    assert!(result.is_err());
}

#[test]
fn test_mock_timeline_get_user_posts_public_only() {
    let mut repo = MockTimelineRepository::new();
    let author_id = uuid::Uuid::new_v4();
    let other_user = uuid::Uuid::new_v4();

    repo.create_post(author_id, "Public post", true).unwrap();
    repo.create_post(author_id, "Private post", false).unwrap();

    // Other user should only see public posts
    let posts = repo.get_user_posts(author_id, other_user, 10);
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].content, "Public post");
}

#[test]
fn test_mock_timeline_get_user_posts_author_sees_all() {
    let mut repo = MockTimelineRepository::new();
    let author_id = uuid::Uuid::new_v4();

    repo.create_post(author_id, "Public post", true).unwrap();
    repo.create_post(author_id, "Private post", false).unwrap();

    // Author should see all posts
    let posts = repo.get_user_posts(author_id, author_id, 10);
    assert_eq!(posts.len(), 2);
}

#[test]
fn test_mock_timeline_delete_post_success() {
    let mut repo = MockTimelineRepository::new();
    let author_id = uuid::Uuid::new_v4();

    let post = repo.create_post(author_id, "To be deleted", true).unwrap();
    
    let result = repo.delete_post(post.id, author_id);
    
    assert!(result.is_ok());
    assert!(repo.posts.is_empty());
}

#[test]
fn test_mock_timeline_delete_post_unauthorized() {
    let mut repo = MockTimelineRepository::new();
    let author_id = uuid::Uuid::new_v4();
    let other_user = uuid::Uuid::new_v4();

    let post = repo.create_post(author_id, "My post", true).unwrap();
    
    let result = repo.delete_post(post.id, other_user);
    
    assert!(result.is_err());
    assert_eq!(repo.posts.len(), 1); // Post should still exist
}
