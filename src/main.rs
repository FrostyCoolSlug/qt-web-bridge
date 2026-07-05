//! Main Entry, Just simple setup and run. We embed the 'Example' UI here so it's all one binary.

mod backend;
pub mod commands;
pub mod dispatcher;
pub mod events;

use backend::Backend;
use include_dir::{Dir, include_dir};
use qtbridge::QApp;
use std::fs;

fn stage_web_assets() -> String {
    static WEB: Dir = include_dir!("$CARGO_MANIFEST_DIR/web");

    let mut dir = std::env::temp_dir();
    dir.push("qt-web-bridge-assets");

    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }

    fs::create_dir_all(&dir).unwrap();

    WEB.extract(&dir).unwrap();

    fs::write(
        dir.join("qwebchannel.js"),
        include_str!(concat!(env!("OUT_DIR"), "/qwebchannel.js")),
    )
    .unwrap();

    let index = dir.join("index.html");
    format!("file://{}", index.to_string_lossy())
}

#[tokio::main]
async fn main() {
    let web_root_url = stage_web_assets();
    backend::set_pending_web_root_url(web_root_url);

    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("../qml/Main.qml"))
        .run();
}
