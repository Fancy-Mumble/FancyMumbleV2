use patch::{Line, Patch};
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::str;
use tokio::runtime::Runtime;

const DOWNLOAD_MUMBLE_PROTO_DIR: &'static str =
    "https://raw.githubusercontent.com/mumble-voip/mumble/82bcd1eb3d53aa9bfc1f6ff539961b0c29336266/src/Mumble.proto";
const MUMBLE_PROTO_SHA256: &'static str =
    "0f86d85938ff2268e3eb05ce0120805fb049ad0d062f4d01c6657b048dcc9245";
const PATCHED_MUMBLE_PROTO_HASH: &'static str =
    "ebadea7bcb720da05149076b1b0ec7a9ff1107a5107a4137b75e8e45fb52f68d";

fn apply(diff: Patch, old: &str) -> String {
    let old_lines = old.lines().collect::<Vec<&str>>();
    let mut out: Vec<&str> = vec![];
    let mut old_line = 0;
    for hunk in diff.hunks {
        while old_line < hunk.old_range.start - 1 {
            out.push(old_lines[old_line as usize]);
            old_line += 1;
        }
        old_line += hunk.old_range.count;
        for line in hunk.lines {
            match line {
                Line::Add(s) | Line::Context(s) => out.push(s),
                Line::Remove(_) => {}
            }
        }
    }
    while old_line < old_lines.len() as u64 {
        out.push(old_lines[old_line as usize]);
        old_line += 1;
    }
    out.push(""); // add a newline at the end
    out.join("\n")
}

fn get_data_hash_str(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let file_hash = hasher.finalize();

    format!("{:x}", file_hash)
}

fn read_file_as_bytes(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    fs::read_to_string(file_path).map_err(|e| e.into())
}

fn write_to_file(data: &[u8], file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(data).expect("Failed to write file");

    Ok(())
}

async fn download_file(
    url: &str,
    sha256: &str,
    file_path: &Path,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client.get(url).send().await?.text().await?;
    let response_bytes = response.as_bytes();

    let file_hash = get_data_hash_str(response_bytes);
    if &file_hash != sha256 {
        return Err(format!(
            "File hash does not match expected hash: {file_hash}, actual hash: {sha256}"
        )
        .into());
    }

    write_to_file(response_bytes, file_path)?;

    Ok(response_bytes.to_vec())
}

fn main() -> io::Result<()> {
    let mumble_proto_path = Path::new("src/proto/Mumble.proto");
    let mumble_proto_patch = Path::new("src/proto/Mumble.proto.patch");

    let mumble_proto_bytes = read_file_as_bytes(mumble_proto_path).unwrap_or_default();
    let hash = get_data_hash_str(&mumble_proto_bytes.as_bytes());

    // download Mumble proto and patch it
    if hash != PATCHED_MUMBLE_PROTO_HASH {
        let rt = Runtime::new()?;
        rt.block_on(async {
            let resonse_file = download_file(
                DOWNLOAD_MUMBLE_PROTO_DIR,
                MUMBLE_PROTO_SHA256,
                mumble_proto_path,
            )
            .await
            .expect("Failed to download Mumble.proto");
            let response_str = str::from_utf8(&resonse_file).expect("Failed to parse response");

            let patch_output = read_file_as_bytes(mumble_proto_patch).expect("Failed to read file");
            let patch = Patch::from_single(patch_output.as_str()).expect("Failed to parse patch");
            let new_content = apply(patch, response_str);
            write_to_file(new_content.as_bytes(), mumble_proto_path).expect("Failed to write file");
        });
    }

    prost_build::compile_protos(&["src/proto/Mumble.proto"], &["src/"])?;
    tauri_build::build();

    Ok(())
}
