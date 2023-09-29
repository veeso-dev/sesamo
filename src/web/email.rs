use actix_web::{post, web, HttpResponse};
use base64::{engine, Engine as _};

use super::WebserverData;

/// Send request payload
#[derive(Debug, Deserialize)]
pub struct Send {
    /// Email recipients
    recipients: Vec<String>,
    /// Email subject
    subject: String,
    /// Base 64 encoded email content
    body: String,
}

#[post("/send")]
pub(super) async fn send(payload: web::Json<Send>, data: web::Data<WebserverData>) -> HttpResponse {
    if payload.recipients.is_empty() {
        return HttpResponse::BadRequest().body("recipients should contain at least one item");
    }

    // decode body
    let body = match engine::general_purpose::STANDARD_NO_PAD
        .decode(&payload.body)
        .map(String::from_utf8)
    {
        Ok(Ok(body)) => body,
        Ok(Err(e)) => {
            error!("failed to decode body as string: {e}");
            return HttpResponse::BadRequest()
                .body("body must be a valid UTF-8 string encoded to Base64");
        }
        Err(e) => {
            error!("failed to decode body: {e}");
            return HttpResponse::BadRequest()
                .body("body must be a valid UTF-8 string encoded to Base64");
        }
    };

    if let Err(err) = data
        .aws_ses_client
        .send(
            &data.email_sender,
            &payload.recipients,
            &payload.subject,
            &body,
        )
        .await
    {
        error!("failed to send mail: {err}");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}
