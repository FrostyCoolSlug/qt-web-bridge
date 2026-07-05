//! Simple build script to copy qwebchannel.js to the output directory

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn qt_prefix() -> String {
    let out = Command::new("pkg-config")
        .args(["--variable=prefix", "Qt6WebChannel"])
        .output()
        .expect("failed to run pkg-config");

    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

fn main() {
    println!("cargo:rerun-if-changed=web");
    println!("cargo:rerun-if-changed=qml");

    let prefix = qt_prefix();
    let js_path = PathBuf::from(prefix).join("share/qt6/webchannel/qwebchannel.js");
    if !js_path.exists() {
        panic!("qwebchannel.js not found in {}", js_path.display());
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest = Path::new(&out_dir).join("qwebchannel.js");

    fs::copy(&js_path, &dest).unwrap_or_else(|e| {
        panic!(
            "failed to copy {} to {}: {e}",
            js_path.display(),
            dest.display()
        )
    });

    println!("cargo:rerun-if-changed={}", js_path.display());
}
