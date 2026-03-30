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
    /// Authentication message
    Auth { token: String },
    
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

/// WebSocket session wrapper
pub struct WsSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session: RwLock<Session>,
    pub last_ping: RwLock<chrono::DateTime<chrono::Utc>>,
}

/// WebSocket server managing all connections
pub struct WebSocketServer {
    sessions: Arc<RwLock<HashMap<Uuid, WsSession>>>,
    user_sessions: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>, // user_id -> session_ids
    notification_tx: mpsc::Sender<WsNotification>,
}

/// Notification to be sent via WebSocket
#[derive(Debug, Clone)]
pub struct WsNotification {
    pub target_user_id: Uuid,
    pub message: WsMessage,
}

impl WebSocketServer {
    pub fn new() -> (Self, mpsc::Receiver<WsNotification>) {
        let (tx, rx) = mpsc::channel(1000);
        
        (
            Self {
                sessions: Arc::new(RwLock::new(HashMap::new())),
                user_sessions: Arc::new(RwLock::new(HashMap::new())),
                notification_tx: tx,
            },
            rx,
        )
    }
    
    /// Add a new session
    pub async fn add_session(&self, session: WsSession) {
        let session_id = session.id;
        let user_id = session.user_id;
        
        // Add to sessions map
        self.sessions.write().await.insert(session_id, session);
        
        // Add to user_sessions map
        self.user_sessions
            .write()
            .await
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(session_id);
            
        // Broadcast online status
        self.broadcast_user_status(user_id, true).await;
    }
    
    /// Remove a session
    pub async fn remove_session(&self, session_id: Uuid) {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.remove(&session_id) {
            let user_id = session.user_id;
            drop(sessions);
            
            // Remove from user_sessions
            let mut user_sessions = self.user_sessions.write().await;
            if let Some(sessions) = user_sessions.get_mut(&user_id) {
                sessions.retain(|&id| id != session_id);
                if sessions.is_empty() {
                    user_sessions.remove(&user_id);
                    // Broadcast offline status
                    drop(user_sessions);
                    self.broadcast_user_status(user_id, false).await;
                }
            }
        }
    }
    
    /// Send message to a specific user (all their sessions)
    pub async fn send_to_user(&self, user_id: Uuid, message: WsMessage) {
        let user_sessions = self.user_sessions.read().await;
        
        if let Some(session_ids) = user_sessions.get(&user_id) {
            let sessions = self.sessions.read().await;
            
            for session_id in session_ids {
                if let Some(session) = sessions.get(session_id) {
                    if let Ok(json) = serde_json::to_string(&message) {
                        let _ = session.session.text(json).await;
                    }
                }
            }
        }
    }
    
    /// Broadcast message to all connected users
    pub async fn broadcast(&self, message: WsMessage) {
        let sessions = self.sessions.read().await;
        
        for (_, session) in sessions.iter() {
            if let Ok(json) = serde_json::to_string(&message) {
                let _ = session.session.text(json).await;
            }
        }
    }
    
    /// Broadcast user online/offline status to their followers
    async fn broadcast_user_status(&self, user_id: Uuid, is_online: bool) {
        // TODO: Get followers from database and notify them
        // For now, just log
        tracing::info!(
            "User {} is now {}",
            user_id,
            if is_online { "online" } else { "offline" }
        );
    }
    
    /// Get notification sender
    pub fn get_notification_sender(&self) -> mpsc::Sender<WsNotification> {
        self.notification_tx.clone()
    }
}

/// WebSocket connection handler
pub async fn websocket_handler(
    req: HttpRequest,
    body: web::Payload,
    srv: web::Data<WebSocketServer>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let (response, session, msg_stream) = actix_ws::handle(&req, body)
        .map_err(|e| AppError::InternalError(format!("WebSocket error: {}", e)))?;
    
    let session_id = Uuid::new_v4();
    let user_id = user.user_id;
    
    let ws_session = WsSession {
        id: session_id,
        user_id,
        session,
        last_ping: chrono::Utc::now(),
    };
    
    srv.add_session(ws_session).await;
    
    // Spawn task to handle WebSocket messages
    let srv_clone = srv.get_ref().clone();
    actix_rt::spawn(async move {
        handle_websocket_messages(msg_stream, srv_clone, session_id, user_id).await;
    });
    
    Ok(response)
}

/// Handle incoming WebSocket messages
async fn handle_websocket_messages(
    mut msg_stream: actix_ws::MessageStream,
    srv: WebSocketServer,
    session_id: Uuid,
    user_id: Uuid,
) {
    while let Some(Ok(msg)) = msg_stream.recv().await {
        match msg {
            Message::Text(text) => {
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    match ws_msg {
                        WsMessage::Ping => {
                            // Send pong back
                            let sessions = srv.sessions.read().await;
                            if let Some(session) = sessions.get(&session_id) {
                                let _ = session.session.text(
                                    serde_json::to_string(&WsMessage::Pong).unwrap()
                                ).await;
                            }
                        }
                        WsMessage::Typing { conversation_id, is_typing, .. } => {
                            // Broadcast typing status to conversation participants
                            // TODO: Get participants and notify them
                            tracing::debug!(
                                "User {} typing in conversation {}: {}",
                                user_id, conversation_id, is_typing
                            );
                        }
                        _ => {}
                    }
                }
            }
            Message::Close(_) => {
                srv.remove_session(session_id).await;
                break;
            }
            _ => {}
        }
    }
    
    // Clean up on disconnect
    srv.remove_session(session_id).await;
}

/// Background task to process notifications
pub async fn notification_processor(
    mut rx: mpsc::Receiver<WsNotification>,
    srv: WebSocketServer,
) {
    while let Some(notification) = rx.recv().await {
        srv.send_to_user(notification.target_user_id, notification.message).await;
    }
}