use std::{error::Error, process};

use rust_completions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = rust_completions::Config::build().unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1)
    });

    if let Err(e) = rust_completions::run(config).await {
        eprintln!("Application error: {e}");
        process::exit(1)
    }

    Ok(())
}
