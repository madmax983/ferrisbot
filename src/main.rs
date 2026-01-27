use ferrisbot::app::{App, AppConfig};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ferrisbot=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Ferrisbot...");

    // Load configuration from environment
    let config = match AppConfig::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::error!("Failed to load configuration: {}", e);
            eprintln!("Error: {}", e);
            eprintln!("\nRequired environment variables:");
            eprintln!("  DISCORD_TOKEN - Discord bot token");
            eprintln!("  ANTHROPIC_API_KEY - Claude API key");
            eprintln!("\nOptional environment variables:");
            eprintln!("  GATEWAY_PORT - Gateway port (default: 18789)");
            std::process::exit(1);
        }
    };

    // Create the application
    let app = App::new(config);

    // Run the full application (gateway + Discord bot)
    if let Err(e) = app.run().await {
        tracing::error!("Application error: {}", e);
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
