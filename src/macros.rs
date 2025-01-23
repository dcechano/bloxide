// Copyright 2025 Bloxide, all rights reserved
// Wait for Ctrl+C macro
#[macro_export]
macro_rules! wait_for_ctrl_c {
    ($($actor:expr),+ $(,)?) => {{
        println!("Press Ctrl+C to exit");
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                println!("\nShutting down...");
                // Send shutdown messages
                $(let _ = $actor.try_send(Message::<StandardPayload>::new(
                    777, // TODO find better solution for message ID
                    StandardPayload::Shutdown));)*
            }
            Err(err) => eprintln!("Unable to listen for shutdown signal: {}", err),
        }
    }};
}
