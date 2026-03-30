//! WebSocket server for real-time updates
//!
//! This module provides WebSocket connections for:
//! - Live notifications
//! - Real-time messaging
//! - Typing indicators
//! - Online/offline status
//! - Live comment updates

use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::{Message, Session};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use crate::middleware::auth::AuthenticatedUser;
use crate::errors::AppError;

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    /// Notification received
    Notification { 
        id: Uuid,
        notification_type: String,
        title: String,
        message: String,
        data: Option<serde_json::Value>,
    },
    
    /// New message in conversation
    NewMessage {
        conversation_id: Uuid,
        message_id: Uuid,
        sender_id: Uuid,
        sender_name: String,
        content: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    
    /// User is typing
    Typing {
        conversation_id: Uuid,
        user_id: Uuid,
        is_typing: bool,
    },
    
    /// User online status changed
    UserStatus {
        user_id: Uuid,
        is_online: bool,
        last_seen: Option<chrono::DateTime<chrono::Utc>>,
    },
    
    /// New comment on post
    NewComment {
        post_id: Uuid,
        comment_id: Uuid,
        author_id: Uuid,
        author_name: String,
        content: String,
    },
    
    /// New post from followed user
    NewPost {
        post_id: Uuid,
        author_id: Uuid,
        author_name: String,
        content_preview: String,
    },
    
    /// Ping/Pong for connection keepalive
    Ping,
    Pong,
    
    /// Error message
    Error { message: String },
}

/// WebSocket server managing all connections
#[derive(Clone)]
pub struct WebSocketServer {
    sessions: Arc<RwLock<HashMap<Uuid, (Uuid, Session)>>>, // session_id -> (user_id, session)
}

impl WebSocketServer {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add a new session
    pub async fn add_session(&self, 
        session_id: Uuid, 
        user_id: Uuid, 
        session: Session
    ) {
        self.sessions.write().await.insert(session_id, (user_id, session));
    }
    
    /// Remove a session
    pub async fn remove_session(&self, session_id: Uuid) {
        self.sessions.write().await.remove(&session_id);
    }
    
    /// Send message to a specific user
    pub async fn send_to_user(
        &self, 
        user_id: Uuid, 
        message: WsMessage
    ) {
        let sessions = self.sessions.read().await;
        
        for (_, (uid, session)) in sessions.iter() {
            if *uid == user_id {
                if let Ok(json) = serde_json::to_string(&message) {
                    let _ = session.clone().text(json).await;
                }
            }
        }
    }
}

/// WebSocket connection handler
pub async fn websocket_handler(
    req: HttpRequest,
    body: web::Payload,
    srv: web::Data<WebSocketServer>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body
    ).map_err(|e| AppError::InternalError(format!("WebSocket error: {}", e)))?;
    
    let session_id = Uuid::new_v4();
    let user_id = user.user_id;
    let srv_clone = srv.get_ref().clone();
    
    srv.add_session(session_id, user_id, session.clone()).await;
    
    // Spawn task to handle messages
    actix_rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                        match ws_msg {
                            WsMessage::Ping => {
                                let _ = session.text(
                                    serde_json::to_string(&WsMessage::Pong).unwrap()
                                ).await;
                            }
                            _ => {}
                        }
                    }
                }
                Message::Close(_) => {
                    srv_clone.remove_session(session_id).await;
                    break;
                }
                _ => {}
            }
        }
        
        srv_clone.remove_session(session_id).await;
    });
    
    Ok(response)
}