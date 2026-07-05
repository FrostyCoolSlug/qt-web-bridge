#[macro_export]
macro_rules! generate_handler {
    ($($cmd:ident),* $(,)?) => {
        pub async fn dispatch_command(
            name: &str,
            args: serde_json::Value
        ) -> Result<serde_json::Value, String> {
            match name {
                $(
                    stringify!($cmd) => $cmd(args).await,
                )*
                _ => Err(format!("Unknown command: {}", name)),
            }
        }
    };
}
