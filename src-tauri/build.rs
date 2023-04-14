use std::io::Result;

fn main() -> Result<()> {
  prost_build::compile_protos(&["src/proto/Mumble.proto"], &["src/"])?;
  tauri_build::build();

  Ok(())
}
