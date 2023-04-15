use serde::ser::{Serialize, SerializeStruct};

use crate::utils::messages::mumble::proto::{TextMessage, Version, UdpTunnel, Authenticate, Ping, Reject, ServerSync, ChannelRemove, ChannelState, UserRemove, UserState, BanList, PermissionDenied, Acl, QueryUsers, CryptSetup, ContextActionModify, ContextAction, UserList, VoiceTarget, PermissionQuery, CodecVersion, UserStats, RequestBlob, ServerConfig, SuggestConfig};

impl Serialize for TextMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("TextMessage", 5)?;

        s.serialize_field("actor", &self.actor)?;
        s.serialize_field("channel_id", &self.channel_id)?;
        s.serialize_field("message", &self.message)?;
        s.serialize_field("session", &self.session)?;
        s.serialize_field("tree_id", &self.tree_id)?;
        s.end()
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Version", 0)?;
        s.end()
    }
}
impl Serialize for UdpTunnel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("UdpTunnel", 0)?;
        s.end()
    }
}
impl Serialize for Authenticate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Authenticate", 0)?;
        s.end()
    }
}
impl Serialize for Ping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Ping", 0)?;
        s.end()
    }
}
impl Serialize for Reject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Reject", 0)?;
        s.end()
    }
}
impl Serialize for ServerSync {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ServerSync", 0)?;
        s.end()
    }
}
impl Serialize for ChannelRemove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ChannelRemove", 0)?;
        s.end()
    }
}
impl Serialize for ChannelState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ChannelState", 0)?;
        s.end()
    }
}
impl Serialize for UserRemove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("UserRemove", 0)?;
        s.end()
    }
}
impl Serialize for UserState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("UserState", 0)?;
        s.end()
    }
}
impl Serialize for BanList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("BanList", 0)?;
        s.end()
    }
}

impl Serialize for PermissionDenied {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("PermissionDenied", 0)?;
        s.end()
    }
}
impl Serialize for Acl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Acl", 0)?;
        s.end()
    }
}
impl Serialize for QueryUsers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("QueryUsers", 0)?;
        s.end()
    }
}
impl Serialize for CryptSetup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("CryptSetup", 0)?;
        s.end()
    }
}
impl Serialize for ContextActionModify {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ContextActionModify", 0)?;
        s.end()
    }
}
impl Serialize for ContextAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ContextAction", 0)?;
        s.end()
    }
}
impl Serialize for UserList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("UserList", 0)?;
        s.end()
    }
}
impl Serialize for VoiceTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("VoiceTarget", 0)?;
        s.end()
    }
}
impl Serialize for PermissionQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("PermissionQuery", 0)?;
        s.end()
    }
}
impl Serialize for CodecVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("CodecVersion", 0)?;
        s.end()
    }
}
impl Serialize for UserStats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("UserStats", 0)?;
        s.end()
    }
}
impl Serialize for RequestBlob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("RequestBlob", 0)?;
        s.end()
    }
}
impl Serialize for ServerConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ServerConfig", 0)?;
        s.end()
    }
}
impl Serialize for SuggestConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("SuggestConfig", 0)?;
        s.end()
    }
}
