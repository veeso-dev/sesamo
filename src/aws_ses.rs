use aws_config::SdkConfig;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};

/// client for AWS SES
pub struct AwsSesClient {
    aws_config: SdkConfig,
}

impl From<SdkConfig> for AwsSesClient {
    fn from(aws_config: SdkConfig) -> Self {
        Self { aws_config }
    }
}

impl AwsSesClient {
    /// Send email to `recipient` with the set
    pub async fn send(
        &self,
        sender: &str,
        recipients: &[String],
        subject: &str,
        content: &str,
    ) -> anyhow::Result<()> {
        let client = aws_sdk_sesv2::Client::new(&self.aws_config);
        debug!("AWS SES client ready");

        let destination = Destination::builder()
            .set_to_addresses(Some(vec![recipients
                .iter()
                .map(|s| s.to_string())
                .collect()]))
            .build();

        let subject_content = Content::builder().data(subject).charset("UTF-8").build();
        let body_content = Content::builder().data(content).charset("UTF-8").build();
        let body = Body::builder().html(body_content).build();

        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        let email_content = EmailContent::builder().simple(msg).build();

        debug!("email content: {content}");
        info!("sending email from {sender} to {recipients:?}");

        if let Err(err) = client
            .send_email()
            .from_email_address(sender)
            .destination(destination)
            .content(email_content)
            .send()
            .await
        {
            error!("Failed to send email: {err:?}");
            anyhow::bail!(err);
        }

        info!("email sent");

        Ok(())
    }
}
