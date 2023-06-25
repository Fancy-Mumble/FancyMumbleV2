pub mod message_router;
pub mod message_transmitter;
pub mod serialize;
pub mod stream_reader;

use std::cmp;

use crate::{mumble, utils::messages::message_builder};
use tauri::PackageInfo;
use tokio::sync::broadcast::Sender;

const OFFSET_MAJOR: u64 = 48;
const OFFSET_MINOR: u64 = 32;
const OFFSET_PATCH: u64 = 16;
const FIELD_MASK: u64 = 0xFFFF;
const FIELD_MAJOR: u64 = FIELD_MASK << OFFSET_MAJOR;
const FIELD_MINOR: u64 = FIELD_MASK << OFFSET_MINOR;
const FIELD_PATCH: u64 = FIELD_MASK << OFFSET_PATCH;

const fn get_major(version: u64) -> u64 {
    (version & FIELD_MAJOR) >> OFFSET_MAJOR
}

const fn get_minor(version: u64) -> u64 {
    (version & FIELD_MINOR) >> OFFSET_MINOR
}

const fn get_patch(version: u64) -> u64 {
    (version & FIELD_PATCH) >> OFFSET_PATCH
}

const fn from_components(major: u64, minor: u64, patch: u64) -> u64 {
    (major << OFFSET_MAJOR) | (minor << OFFSET_MINOR) | (patch << OFFSET_PATCH)
}

// The size of the version field in the legacy protocol is 32 bits, so we need to truncate
#[allow(clippy::cast_possible_truncation)]
fn to_legacy_version(version: u64) -> u32 {
    // If any of the version components exceeds their allowed value range, they will
    // be truncated to the highest representable value
    let major = u32::from(cmp::min(get_major(version) as u16, u16::MAX)) << 16;
    let minor = u32::from(cmp::min(get_minor(version) as u8, u8::MAX)) << 8;
    let patch = u32::from(cmp::min(get_patch(version) as u8, u8::MAX));

    major | minor | patch
}

pub fn init_connection(username: &str, channel: &Sender<Vec<u8>>, package_info: &PackageInfo) {
    let version = from_components(
        package_info.version.major + 2,
        package_info.version.minor,
        package_info.version.patch,
    );

    let mumble_version = from_components(1, 5, 0);

    let info = os_info::get();

    let version = mumble::proto::Version {
        version_v1: Some(to_legacy_version(mumble_version)),
        os: Some(format!(
            "{} {} ({} - {})",
            info.os_type(),
            info.version(),
            info.architecture().unwrap_or("x86_64"),
            info.bitness()
        )),
        os_version: Some(info.version().to_string()),
        release: Some(package_info.package_name()),
        version_v2: Some(version),
        fancy_version: Some(version),
    };

    let buffer = message_builder(&version).unwrap_or_default();
    _ = channel.send(buffer);

    let auth = mumble::proto::Authenticate {
        opus: Some(true),
        celt_versions: vec![-2_147_483_632, -2_147_483_637],
        password: None,
        tokens: vec![],
        username: Some(username.to_string()),
        client_type: Some(0), // 1 = BOT, 0 = User
    };

    let buffer = message_builder(&auth).unwrap_or_default();
    _ = channel.send(buffer);
}
