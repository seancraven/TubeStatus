mod sms_interaction;
mod tfl_status;
mod tube;
use sms_interaction::twilio_interface;
use tokio;
use tube::{Line, Lines};

#[tokio::main]
async fn main() {
    // Basic idea:
    // store a small local database of users, who each have a list
    // of associated lines and time for a message,
    // User requests to use the service by sending tubepotato a message/email
    // they are then registered with the service. With there preferences
    // On the time checks that the user requests the tube status is sent to their
    // phone number.
    //
    //
    // Components of this idea;
    // 1) webscraper for the tfl website. I assume that this will be the most brittle
    // part.
    // The classes are well delimited.

    // 2) User database, write and read users.
    //  rough schema
    //  users: a, b ,c
    //  each user has own table,
    //  User a:
    //  request time,
    //  lines,
    // 3) Interface with messenger, and request handeling.
    //
    //
    // TODO: Debug this.
    let angie = "+447478670019";
    let mut lines = Lines::new();
    let jubilee = Line::Jubilee;
    lines.update().await;
    let jubilee_status = lines
        .get(&jubilee)
        .expect("Expected to get info about jubilee");

    let message_body = format!(
        "Hi Angie, a short summary of jubilee service {:?}",
        jubilee_status.short,
    );
    let message = twilio_interface::send_message(angie, &message_body).await;
}
