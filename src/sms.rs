use dotenv::dotenv;
use std::env;
use twilio::{Client, OutboundMessage, TwilioError};
/// Function used to send a message through the twilio api.
///
///  # Arguments
///     ## authorized_recipient: a phone number which
pub async fn message(authorized_recipient: &str, message_content: &str) -> Result<(), TwilioError> {
    dotenv().ok();
    let from = env::var("PHONE_FROM").expect("Expected a phone number to send from");
    let app_id = env::var("ACCOUNT_SID").expect("Expected an account sid");
    let auth_token = env::var("ACCOUNT_TOKEN").expect("Expected an account auth token");
    let client = Client::new(&app_id, &auth_token);
    let msg = OutboundMessage::new(&from, authorized_recipient, message_content);
    client.send_message(msg).await?;
    Ok(())
}
