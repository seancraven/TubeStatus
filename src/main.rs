mod lines;
mod scraper;
use crate::lines::structs::*;
use crate::scraper::*;

fn main() {
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
    let lizzy_line = Line::ElizabethLine;
    let lizzy_day = TubeLine {
        name: lizzy_line,
        status_message: "dave",
    };
    println!("TFL updates are coming");
    scrape::scrape().unwrap();
}
