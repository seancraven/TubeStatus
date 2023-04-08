pub mod twilio_interface {
    use dotenv::dotenv;
    use std::env;
    use twilio::{Client, OutboundMessage};

    /// Function used to send a message through the twilio api.
    ///
    /// The function depends on a .env file containing the following variables:
    ///    * `PHONE_FROM` - The phone number of the sender.
    ///    * `ACCOUNT_SID` - The account sid of the twilio account.
    ///    * `ACCOUNT_TOKEN` - The account auth token of the twilio account.
    ///  # Arguments
    ///     * `authorized_recipient` - The phone number of the recipient.
    ///     They must be authorized to receive messages from the twilio account.
    ///     * `message_content` - The content of the message.
    pub async fn send_message(authorized_recipient: &str, message_content: &str) {
        dotenv().ok();
        let from = env::var("PHONE_FROM").expect("PHONE_FROM not set");
        let app_id = env::var("ACCOUNT_SID").expect("ACCOUNT_SID not set");
        let auth_token = env::var("ACCOUNT_TOKEN").expect("ACCOUNT_TOKEN not set");
        let client = Client::new(&app_id, &auth_token);
        let msg = OutboundMessage::new(&from, authorized_recipient, message_content);
        // TODO: Handle errors, remove prints? not sure.
        match client.send_message(msg).await {
            Ok(m) => println!("{:?}", m),
            Err(e) => eprintln!("{:?}", e),
        };
    }
}
