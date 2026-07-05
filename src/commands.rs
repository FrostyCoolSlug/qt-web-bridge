//! Test commands and their registration

use crate::events::emit;
use crate::generate_handler;
use command_macros::command;
use serde_json::json;

#[command]
pub async fn echo(text: String) -> String {
    text
}

#[command]
pub async fn upper(text: String) -> String {
    emit("upper-triggered", json!({"text": text}));
    text.to_uppercase()
}

// registers into dispatcher
generate_handler![echo, upper];
