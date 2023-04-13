use crate::tube::Line;
use chrono::NaiveTime;
use sqlx::mysql::MySqlPool;
use std::collections::HashMap;

/// A user in the database.
/// The phone number is the primary key and the fk, for the other info.
///
pub struct Recipient {
    user: User,
    days: DaysDB,
    lines: LinesDB,
}
pub struct User {
    phone_number: u64,
    update_time: NaiveTime,
}
pub struct DaysDB {
    /// Which days the user wants to be notified about.
    phone_number: String,
    map: HashMap<String, bool>,
}
pub struct LinesDB {
    /// Which lines the user wants to be notified about.
    phone_number: String,
    map: HashMap<Line, bool>,
}
impl LinesDB {
    pub fn new(phone_number: String) -> LinesDB {
        let mut map = HashMap::with_capacity(15);
        map.insert(Line::ElizabethLine, false);
        map.insert(Line::HammersmithCity, false);
        map.insert(Line::Jubilee, false);
        map.insert(Line::Metropolitan, false);
        map.insert(Line::Bakerloo, false);
        map.insert(Line::Central, false);
        map.insert(Line::Circle, false);
        map.insert(Line::District, false);
        map.insert(Line::Northern, false);
        map.insert(Line::Piccadilly, false);
        map.insert(Line::Victoria, false);
        map.insert(Line::WaterlooCity, false);
        map.insert(Line::LondonOverground, false);
        map.insert(Line::DLR, false);
        map.insert(Line::Tram, false);
        LinesDB { phone_number, map }
    }
    pub fn set_line(&mut self, line: Line, value: bool) {
        self.map.insert(line, value);
    }
    pub fn get_line(&self, line: Line) -> bool {
        self.map.get(&line).unwrap().clone()
    }
    pub fn get_phone_number(&self) -> String {
        self.phone_number.clone()
    }
    pub async fn insert_into_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let mut query = String::from("INSERT INTO lines (phone_number");
        let mut values = format!("VALUES ('{}'", self.phone_number);
        for (line, to_update) in self.map.iter() {
            if *to_update {
                query.push_str(&format!(", {}", line.name()));
                values.push_str(",TRUE");
                // This leaves final values with one extra comma
                // Think this is fine.
            }
        }
        query.push_str(") ");
        values.push_str(");");
        query.push_str(&values);
        println!("{}", &query);
        sqlx::query(&query).execute(pool).await?;
        Ok(())
    }
}
impl DaysDB {
    pub fn new(phone_number: String) -> DaysDB {
        let mut map = HashMap::with_capacity(7);
        map.insert(String::from("Monday"), false);
        map.insert(String::from("Tuesday"), false);
        map.insert(String::from("Wednesday"), false);
        map.insert(String::from("Thursday"), false);
        map.insert(String::from("Friday"), false);
        map.insert(String::from("Saturday"), false);
        map.insert(String::from("Sunday"), false);
        DaysDB { phone_number, map }
    }
    pub fn set_day(&mut self, day: String, value: bool) {
        self.map.insert(day, value);
    }
    pub fn get_day(&self, day: String) -> bool {
        self.map.get(&day).unwrap().clone()
    }
    pub fn get_phone_number(&self) -> String {
        self.phone_number.clone()
    }
    pub async fn insert_into_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let mut query = String::from("INSERT INTO days (phone_number");
        let mut values = format!("VALUES ('{}'", self.phone_number);
        for (day, to_update) in self.map.iter() {
            if *to_update {
                query.push_str(&format!(",{}", day));
                values.push_str(", TRUE ");
            }
        }
        query.push_str(") ");
        values.push_str(");");
        query.push_str(&values);
        println!("Query {}", &query);
        sqlx::query(&query).execute(pool).await?;
        Ok(())
    }
}
impl User {
    pub fn new(phone_number: String, update_time: NaiveTime) -> User {
        User {
            phone_number,
            update_time,
        }
    }
    pub fn get_phone_number(&self) -> String {
        self.phone_number.clone()
    }
    pub fn get_update_time(&self) -> NaiveTime {
        self.update_time.clone()
    }
    pub async fn insert_into_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (phone_number, update_time) VALUES (?, ?)",
            self.phone_number,
            self.update_time,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
impl Recipient {
    pub fn new(phone_number: String, update_time: NaiveTime) -> Recipient {
        Recipient {
            user: User::new(phone_number.clone(), update_time),
            days: DaysDB::new(phone_number.clone()),
            lines: LinesDB::new(phone_number.clone()),
        }
    }
    pub async fn insert_into_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let (user,) = tokio::join!(self.user.insert_into_db(pool));
        user?;

        let (days, lines) = tokio::join!(
            self.days.insert_into_db(pool),
            self.lines.insert_into_db(pool)
        );
        days?;
        lines?;
        Ok(())
    }
}
async fn create_tabels(conn: &MySqlPool) {
    let users = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            phone_number BIGINT NOT NULL UNIQUE,
            update_time TIME NOT NULL,
            CONSTRAINT pk_user PRIMARY KEY (phone_number)
        )
        "#,
    )
    .execute(conn);

    let days = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS days (
            phone_number BIGINT NOT NULL UNIQUE,
            Monday BOOLEAN NOT NULL DEFAULT FALSE,
            Tuesday BOOLEAN NOT NULL DEFAULT FALSE,
            Wednesday BOOLEAN NOT NULL DEFAULT FALSE,
            Thursday BOOLEAN NOT NULL DEFAULT FALSE, 
            Friday BOOLEAN NOT NULL DEFAULT FALSE,
            Saturday BOOLEAN NOT NULL DEFAULT FALSE,
            Sunday BOOLEAN NOT NULL DEFAULT FALSE,
            CONSTRAINT fk_days FOREIGN KEY (phone_number) REFERENCES users(phone_number)
        )
        "#,
    )
    .execute(conn);

    let lines = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS tube_lines (
            phone_number BIGINT NOT NULL UNIQUE,
            Elizabeth BOOLEAN NOT NULL DEFAULT FALSE,
            Jubilee BOOLEAN NOT NULL DEFAULT FALSE,
            Bakerloo BOOLEAN NOT NULL DEFAULT FALSE,
            Central BOOLEAN NOT NULL DEFAULT FALSE,
            Circle BOOLEAN NOT NULL DEFAULT FALSE,
            District BOOLEAN NOT NULL DEFAULT FALSE,
            DLR BOOLEAN NOT NULL DEFAULT FALSE,
            Hammersmith BOOLEAN NOT NULL DEFAULT FALSE,
            Metropolitan BOOLEAN NOT NULL DEFAULT FALSE,
            Northern BOOLEAN NOT NULL DEFAULT FALSE,
            Piccadilly BOOLEAN NOT NULL DEFAULT FALSE,
            Victoria BOOLEAN NOT NULL DEFAULT FALSE,
            Waterloo BOOLEAN NOT NULL DEFAULT FALSE,
            Overground BOOLEAN NOT NULL DEFAULT FALSE, 
            Tram BOOLEAN NOT NULL DEFAULT FALSE,
            CONSTRAINT fk_lines FOREIGN KEY (phone_number) REFERENCES users(phone_number)
        )
        "#,
    )
    .execute(conn);

    let (r_1, r_2, r_3) = tokio::join!(users, days, lines);
    r_1.expect("Failed to create users table");
    r_2.expect("Failed to create days table");
    r_3.expect("Failed to create lines table");
}
#[cfg(test)]
mod dbtests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn crate_tabel_test() {
        dotenv().ok();
        let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        create_tabels(&pool).await;
    }

    #[tokio::test]
    async fn user_test() {
        dotenv().ok();
        let mut recipient = Recipient::new(
            String::from("0712345671"),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        recipient.days.set_day(String::from("Monday"), true);
        recipient.lines.set_line(Line::Jubilee, true);
        sqlx::query!(
            "insert into users (phone_number, update_time) values ('1', ?)",
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        )
        .execute(&pool)
        .await
        .expect("failed to do the test");
        sqlx::query!("insert into days (phone_number) values ('1')",)
            .execute(&pool)
            .await
            .expect("failed to do the test");
        sqlx::query!("update days set Monday = TRUE where phone_number = '1'",)
            .execute(&pool)
            .await
            .expect("failed to do the test");
        recipient
            .insert_into_db(&pool)
            .await
            .expect("failed to insert recitp");
    }
}
