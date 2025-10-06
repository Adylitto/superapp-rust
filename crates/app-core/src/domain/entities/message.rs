use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Message entity for real-time chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub conversation_id: Uuid,
    pub content: MessageContent,
    pub is_encrypted: bool,
    pub is_read: bool,
    pub encryption_metadata: Option<EncryptionMetadata>,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    Text(String),
    Image { url: String, thumbnail: String },
    Video { url: String, duration: u32 },
    File { url: String, filename: String, size: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionMetadata {
    pub algorithm: String,
    pub key_id: String,
    pub nonce: Vec<u8>,
}

impl Message {
    /// Create a new text message
    pub fn new_text(
        sender_id: Uuid,
        recipient_id: Uuid,
        conversation_id: Uuid,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender_id,
            recipient_id,
            conversation_id,
            content: MessageContent::Text(content),
            is_encrypted: false,
            is_read: false,
            encryption_metadata: None,
            created_at: Utc::now(),
            read_at: None,
        }
    }

    /// Mark message as encrypted
    pub fn set_encrypted(&mut self, metadata: EncryptionMetadata) {
        self.is_encrypted = true;
        self.encryption_metadata = Some(metadata);
    }

    /// Mark message as read
    pub fn mark_as_read(&mut self) {
        self.is_read = true;
        self.read_at = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let sender = Uuid::new_v4();
        let recipient = Uuid::new_v4();
        let conversation = Uuid::new_v4();

        let message = Message::new_text(
            sender,
            recipient,
            conversation,
            "Hello!".to_string(),
        );

        assert_eq!(message.sender_id, sender);
        assert!(!message.is_encrypted);
        assert!(!message.is_read);
    }
}
