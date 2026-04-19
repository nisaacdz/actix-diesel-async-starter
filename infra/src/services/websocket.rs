use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::sync::{mpsc, mpsc::error::SendError};
use uuid::Uuid;

pub type SocketSession = mpsc::UnboundedSender<String>;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsEmitEvent<T> {
    pub event: &'static str,
    pub payload: T,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsReceiveEvent<T> {
    pub event: String,
    pub payload: T,
}

#[derive(Debug, derive_more::Display)]

pub enum WsError {
    #[display("Failed to acquire lock: {}", _0)]
    LockError(String),
    #[display("Failed to serialize data: {}", _0)]
    SerializationError(serde_json::Error),
    #[display("Failed to deserialize data: {}", _0)]
    DeserializationError(String),
    #[display("Failed to send message: {}", _0)]
    SendError(SendError<String>),
    #[display("Not found: {}", _0)]
    ReceiverOffline(String),
}

impl std::error::Error for WsError {}

type SocketMap = Arc<RwLock<HashMap<Uuid, SocketSession>>>;

#[derive(Default)]
pub struct WebSocketService {
    pub(crate) socket_map: SocketMap,
}

impl WebSocketService {
    pub fn new() -> Self {
        WebSocketService {
            socket_map: SocketMap::default(),
        }
    }

    pub fn emit_event(
        &self,
        user_id: &Uuid,
        event_name: &'static str,
        payload: impl serde::Serialize,
    ) -> Result<(), WsError> {
        let sender = self
            .socket_map
            .read()
            .map_err(|_| WsError::LockError("Failed to acquire lock for users map".to_string()))?
            .get(user_id)
            .cloned();

        if let Some(sender) = sender {
            let event = WsEmitEvent {
                event: event_name,
                payload,
            };
            let message = serde_json::to_string(&event).map_err(WsError::SerializationError)?;
            sender.send(message).map_err(WsError::SendError)?;
            Ok(())
        } else {
            Err(WsError::ReceiverOffline(format!(
                "User with ID {} is offline",
                user_id
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_send_to_user_success() {
        let socket_service = WebSocketService::new();
        let (tx, mut rx) = mpsc::unbounded_channel();
        let user_id = Uuid::new_v4();

        socket_service
            .socket_map
            .write()
            .unwrap()
            .insert(user_id, tx);

        let payload = "Hello, User!".to_string();
        let result = socket_service.emit_event(&user_id, "message", payload.clone());

        assert!(result.is_ok());

        let received = rx.recv().await.unwrap();
        let expected_event =
            serde_json::from_str::<super::WsReceiveEvent<String>>(&received).unwrap();

        assert_eq!(expected_event.event, "message");
        assert_eq!(&expected_event.payload, &payload);
    }

    #[tokio::test]
    async fn test_send_to_user_not_found() {
        let socket_service = WebSocketService::new();
        let user_id = Uuid::new_v4();
        let msg = "Should fail".to_string();

        let result = socket_service.emit_event(&user_id, "message", msg);

        assert!(result.is_err());
    }
}
