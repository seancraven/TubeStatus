mod database;
mod sms;
mod tfl_status;
mod tube;

#[tokio::main]
async fn main() {
    // fn main() {
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
}
#[cfg(test)]
mod test {
    use crate::sms;
    use crate::tube::{Line, Lines};
    use dotenv::dotenv;
    #[test]
    fn message_test() {
        dotenv().ok();
        let recipient = std::env::var("PHONE_TO").expect("Expected to get recipient env var");
        let mut lines = Lines::new();
        let jubilee = Line::Jubilee;
        lines.update();
        let jubilee_status = lines
            .get(&jubilee)
            .expect("Expected to get info about jubilee");
        let message_body = format!("Hi Person, \n\n{}", jubilee_status,);
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(sms::message(&recipient, &message_body))
            .expect("Failed to send message");
        println!("Sent message");
        assert!(true);
    }
}
