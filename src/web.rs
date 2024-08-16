mod email;
mod health_check;

use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App as ActixApp, HttpServer};
use aws_config::SdkConfig;

use crate::aws_ses::AwsSesClient;

pub struct WebServer {
    server: Server,
}

struct WebserverData {
    aws_ses_client: AwsSesClient,
    email_sender: String,
}

impl WebServer {
    /// Initialize web server
    pub async fn init(
        listen_url: &str,
        aws_config: SdkConfig,
        email_sender: &str,
    ) -> anyhow::Result<Self> {
        info!("webserver initialized");
        info!("listen_url: {listen_url}");

        let listener = TcpListener::bind(listen_url)?;

        let server = {
            let email_sender = email_sender.to_string();
            HttpServer::new(move || {
                let web_data = Data::new(WebserverData {
                    aws_ses_client: AwsSesClient::from(aws_config.clone()),
                    email_sender: email_sender.clone(),
                });
                ActixApp::new()
                    .service(email::send)
                    .service(health_check::check_action)
                    .app_data(web_data)
                    .wrap(Logger::default())
            })
            .listen(listener)?
            .run()
        };

        info!("web server initialized");
        Ok(Self { server })
    }

    /// Run web server
    pub async fn run(self) -> anyhow::Result<()> {
        info!("running web server");
        self.server.await?;
        info!("web server stopped");
        Ok(())
    }
}
