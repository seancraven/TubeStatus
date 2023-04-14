use dotenv::dotenv;
use std::env;
use twilio::{Client, OutboundMessage, TwilioError};
/// Function used to send a message through the twilio api.
///
///  # Arguments
///     ## authorized_recipient: a phone number which
///     ## message_content: the message to be sent.
pub async fn message(
    authorized_recipient: String,
    message_content: String,
) -> Result<(), TwilioError> {
    // I don't like that both of these functions load from the same .env file. might be okay.
    dotenv().ok();
    let from = env::var("PHONE_FROM").expect("Expected a phone number to send from");
    let client = get_twilio_client();
    // Might want to move client out, depending on if other functions need it.
    let msg = OutboundMessage::new(&from, &authorized_recipient, &message_content);
    client.send_message(msg).await?;
    Ok(())
}

pub fn get_twilio_client() -> Client {
    dotenv().ok();
    let app_id = env::var("ACCOUNT_SID").expect("Expected an account sid");
    let auth_token = env::var("ACCOUNT_TOKEN").expect("Expected an account auth token");
    let client = Client::new(&app_id, &auth_token);
    return client;
}
