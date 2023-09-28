#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

mod config;
mod web;

const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/*
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("rust-template v{} - developed by {}", APP_VERSION, APP_AUTHORS);
    let config = config::Config::try_from_env()?;
    info!("initializing web service...");
    let web_service = web::WebServer::init(
        &config.clamav_address,
        config.max_upload_size,
        config.web_port,
    )
    .await?;
    info!("web service OK; running web server...");
    web_service.run().await?;

    Ok(())
}
*/

/*
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("rust-template v{} - developed by {}", APP_VERSION, APP_AUTHORS);
    let config = config::Config::try_from_env()?;

    Ok(())
}
*/

fn main() -> anyhow::Result<()> {
    Ok(())
}
