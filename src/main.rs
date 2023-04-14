mod sms;
mod tfl_status;
mod tube;
mod userdb;
use chrono::Local;
use std::thread::sleep;
use tokio::task::JoinSet;
use tube::Lines;
use userdb::Day;

#[tokio::main]
async fn main() {
    // Start off the infinite loop.
    // This is really bad, There are cases where the loop misses updates.
    let mut current_time = Local::now().naive_local().time();
    let mut next_hour: chrono::NaiveTime;
    let pool = userdb::create_pool()
        .await
        .expect("Failed to create Database pool");
    loop {
        current_time = Local::now().naive_local().time();
        next_hour = current_time + chrono::Duration::hours(1);
        // Get the recipients, who need an update in the next hour.
        let recipients_opt =
            match userdb::recipients_to_update(&pool, &current_time, &next_hour).await {
                Ok(recipients) => recipients,
                Err(e) => {
                    println!("Failed to get recipients: {}", e);
                    None
                }
            };
        let recipients = recipients_opt.unwrap_or(vec![]);
        // Perfom webscrape. to get data.
        let mut line_status = Lines::new();
        line_status.update().await;
        // Container for the async message sending.
        let mut message_futures = JoinSet::new();

        // Iterate through the recipients, and send them a message.
        for rec in recipients.into_iter() {
            //
            let mut message_body = String::from("Hi,\n Line status updates:\n");
            let lines_of_interest = rec.get_lines();
            let days_to_report = rec.get_days();

            if days_to_report.contains(&Day::current_day()) {
                for line in lines_of_interest.into_iter() {
                    let _ = match line_status.get(&line) {
                        Some(line_status) => {
                            message_body.push_str(&format!(
                                "{}: {}\n",
                                line.name(),
                                line_status.short
                            ));
                        }
                        None => {
                            message_body
                                .push_str(&format!("{}: No status available\n", line.name()));
                        }
                    };
                }
            }

            // Send message to user.
            message_futures.spawn(sms::message(rec.get_number(), message_body));
        }
        while let Some(message_response) = message_futures.join_next().await {
            match message_response {
                Ok(message_response) => match message_response {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Failed to send message: {}", e);
                    }
                },
                Err(e) => {
                    println!("Failed to send message: {}", e);
                }
            }
        }
        sleep(std::time::Duration::from_secs(60 * 60));
    }

    // Fetch users from database, that want an update in the
    // next hour.
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::tube::Line;
    use crate::userdb::create_pool;
    use crate::userdb::{Day, Recipient};
    use dotenv::dotenv;
    #[tokio::test]
    async fn main_test() {
        dotenv().ok();
        // First add my number to the database,

        let pool = &create_pool().await.expect("Failed to create Database pool");
        let test_time = chrono::Local::now().naive_local().time() + chrono::Duration::minutes(1);
        let number_b = std::env::var("PHONE_TO").expect("Failed to get phone number");
        let number_a = std::env::var("PHONE_TO_A").expect("Failed to get phone number");
        let recip_a = Recipient::new(
            number_a,
            test_time,
            Some(vec![
                Day::Monday,
                Day::Tuesday,
                Day::Wednesday,
                Day::Thursday,
                Day::Friday,
            ]),
            Some(vec![Line::Bakerloo, Line::Central, Line::Jubilee]),
        );
        let recip_b = Recipient::new(
            number_b,
            test_time,
            Some(vec![
                Day::Monday,
                Day::Tuesday,
                Day::Wednesday,
                Day::Thursday,
                Day::Friday,
            ]),
            Some(vec![Line::Bakerloo, Line::Central, Line::Jubilee]),
        );
        recip_a
            .remove_from_db(pool)
            .await
            .expect("Failed to remove from db");
        recip_b
            .remove_from_db(pool)
            .await
            .expect("Failed to remove from db");
        recip_a
            .insert_into_db(pool)
            .await
            .expect("Failed to insert into db");
        recip_b
            .insert_into_db(pool)
            .await
            .expect("Failed to insert into db");
        println!("Starting Test");
        main();
    }
}
