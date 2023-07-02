use super::*;
use dotenv::dotenv;

/// Tetst to check that a person can be inserted into the db.
#[tokio::test]
async fn user_test() {
    dotenv().ok();
    // Define the recip
    let mut recipient = Recipient::new(
        String::from("test"),
        NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        None,
        None,
    );

    let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    // Make sure there is not old version from crashed tests.
    match recipient.remove_from_db(&pool).await {
        Ok(_) => (),
        Err(_) => (),
    }
    recipient.days.set_day(Day::Monday, true);
    recipient.lines.set_line(Line::Jubilee, true);
    recipient
        .insert_into_db(&pool)
        .await
        .expect("failed to insert recitp");
    let fetched_recipient = Recipient::fetch(recipient.user.phone_number.clone(), &pool)
        .await
        .expect("failed to fetch status");
    // Tidy up table.
    recipient
        .remove_from_db(&pool)
        .await
        .expect("failed to remove Recipient");
    assert_eq!(fetched_recipient, recipient);
}
