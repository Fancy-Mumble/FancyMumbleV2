use std::path::PathBuf;

use xshell::{cmd, Shell};

pub fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    let res = PathBuf::from(dir).to_owned();
    res
}

#[test]
fn check_code_formatting() {
    let sh = &Shell::new().unwrap();
    sh.change_dir(project_root());

    let out = cmd!(sh, "rustup run stable rustfmt --version")
        .read()
        .unwrap();
    if !out.contains("stable") {
        panic!(
            "Failed to run rustfmt from toolchain 'stable'. \
                 Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }

    let res = cmd!(sh, "rustup run stable cargo fmt -- --check").run();
    if res.is_err() {
        let _ = cmd!(sh, "rustup run stable cargo fmt").run();
    }
    res.unwrap()
}
