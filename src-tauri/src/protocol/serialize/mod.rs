use serde::ser::{Serialize, SerializeStruct};

use crate::utils::messages::mumble::proto::TextMessage;

impl Serialize for TextMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut s = serializer.serialize_struct("TextMessage", 5)?;

            s.serialize_field("actor", &self.actor)?;
            s.serialize_field("channel_id", &self.channel_id)?;
            s.serialize_field("message", &self.message)?;
            s.serialize_field("session", &self.session)?;
            s.serialize_field("tree_id", &self.tree_id)?;
            s.end()
    }
}