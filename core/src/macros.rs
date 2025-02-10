// Copyright 2025 Bloxide, all rights reserved
// Wait for Ctrl+C macro
#[macro_export]
macro_rules! wait_for_ctrl_c {
    ($($actor:expr),+ $(,)?) => {{
        println!("Press Ctrl+C to exit");
        match ::tokio::signal::ctrl_c().await {
            Ok(()) => {
                println!("\nShutting down...");
                // Send shutdown messages
                $($actor.try_send(Message::<StandardPayload<R>>::new(
                    0,
                    StandardPayload::Shutdown))
                    .unwrap_or_else(|e| error!("Failed to send Shutdown: {:?}", e));)*
            }
            Err(err) => eprintln!("Unable to listen for shutdown signal: {}", err),
        }
    }};
}
