#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

mod aws_ses;
mod config;
#[cfg(test)]
mod test;
mod web;

use std::io::Write;
use std::path::Path;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("{APP_NAME} v{APP_VERSION} - developed by {APP_AUTHORS}");
    let config = config::Config::try_from_env()?;
    info!("initializing Aws Config...");
    let aws_config = aws_config::load_from_env().await;
    info!(
        "AWS configuration loaded for region: {:?}",
        aws_config.region()
    );
    info!("initializing web service...");
    let web_service =
        web::WebServer::init(&config.listen_url, aws_config, &config.email_sender).await?;

    #[cfg(target_family = "unix")]
    if let Some(pidfile) = config.pidfile.as_deref() {
        debug!("writing pidfile to {}", pidfile.display());
        write_pidfile(pidfile)?;
        info!("pidfile written");
    }

    info!("web service OK; running web server...");
    web_service.run().await?;

    Ok(())
}

#[cfg(target_family = "unix")]
fn write_pidfile(pidfile: &Path) -> anyhow::Result<()> {
    let pid = std::process::id();

    let mut f = std::fs::File::create(pidfile)?;
    writeln!(&mut f, "{pid}")?;

    Ok(())
}
