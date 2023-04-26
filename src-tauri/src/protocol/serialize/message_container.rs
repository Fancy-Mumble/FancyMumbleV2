use serde::Serialize;
use tracing::error;

#[derive(Debug, Serialize)]
pub struct FrontendMessage<T: Serialize + Clone> {
    pub message_type: String,
    pub data: T,
}

impl<T: Serialize + Clone> FrontendMessage<T> {
    pub fn new(message_type: &str, data: T) -> FrontendMessage<T> {
        FrontendMessage {
            message_type: message_type.to_string(),
            data,
        }
    }
}
