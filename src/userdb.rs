// File to handle interation with the database.
// The interface is all defined by a recipient.
// These contain phone number, update time, days and tube lines, to be updated on.
use crate::tube::Line;
use chrono::NaiveTime;
use sqlx::mysql::MySqlPool;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl Day {
    fn to_string(&self) -> String {
        match self {
            Day::Monday => String::from("Monday"),
            Day::Tuesday => String::from("Tuesday"),
            Day::Wednesday => String::from("Wednesday"),
            Day::Thursday => String::from("Thursday"),
            Day::Friday => String::from("Friday"),
            Day::Saturday => String::from("Saturday"),
            Day::Sunday => String::from("Sunday"),
        }
    }
}

/// A user in the database.
/// The phone number is the primary key and the fk, for the other info.
///
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Recipient {
    user: User,
    days: DaysDB,
    lines: LinesDB,
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct User {
    phone_number: String,
    update_time: NaiveTime,
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct DaysDB {
    /// Which days the user wants to be notified about.
    phone_number: String,
    map: HashMap<Day, bool>,
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct LinesDB {
    /// Which lines the user wants to be notified about.
    phone_number: String,
    map: HashMap<Line, bool>,
}
impl LinesDB {
    fn new(phone_number: String, lines: Option<impl IntoIterator<Item = Line>>) -> LinesDB {
        let mut map = HashMap::with_capacity(15);
        match lines {
            Some(lines) => {
                for line in lines {
                    map.insert(line, true);
                }
            }
            None => {}
        };

        LinesDB { phone_number, map }
    }
    fn set_line(&mut self, line: Line, value: bool) {
        self.map.insert(line, value);
    }
    fn get_line(&self, line: Line) -> bool {
        self.map.get(&line).unwrap().clone()
    }
    fn get_phone_number(&self) -> String {
        self.phone_number.clone()
    }
    async fn insert_into_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let mut query = String::from("INSERT INTO tube_lines (phone_number");
        let mut values = format!("VALUES ('{}'", self.phone_number);
        for (line, to_update) in self.map.iter() {
            if *to_update {
                query.push_str(&format!(", {}", line.name()));
                values.push_str(", TRUE");
                // This leaves final values with one extra comma
                // Think this is fine.
            } else {
                query.push_str(&format!(", {}", line.name()));
                values.push_str(", FALSE");
            }
        }
        query.push_str(") ");
        values.push_str(");");
        query.push_str(&values);
        sqlx::query(&query).execute(pool).await?;
        Ok(())
    }
    async fn fetch(phone_number: &String, pool: &MySqlPool) -> Result<LinesDB, sqlx::Error> {
        let mut lines = HashMap::with_capacity(15);
        let query = sqlx::query!(
            r#"SELECT * FROM tube_lines WHERE phone_number = ?"#,
            phone_number
        )
        .fetch_one(pool)
        .await?;
        if query.Bakerloo == 1 {
            lines.insert(Line::Bakerloo, true);
        }
        if query.Central == 1 {
            lines.insert(Line::Central, true);
        }
        if query.Circle == 1 {
            lines.insert(Line::Circle, true);
        }
        if query.District == 1 {
            lines.insert(Line::District, true);
        }
        if query.DLR == 1 {
            lines.insert(Line::DLR, true);
        }
        if query.Elizabeth == 1 {
            lines.insert(Line::ElizabethLine, true);
        }
        if query.Hammersmith == 1 {
            lines.insert(Line::HammersmithCity, true);
        }
        if query.Jubilee == 1 {
            lines.insert(Line::Jubilee, true);
        }
        if query.Overground == 1 {
            lines.insert(Line::LondonOverground, true);
        }
        if query.Metropolitan == 1 {
            lines.insert(Line::Metropolitan, true);
        }
        if query.Northern == 1 {
            lines.insert(Line::Northern, true);
        }
        if query.Piccadilly == 1 {
            lines.insert(Line::Piccadilly, true);
        }
        if query.Tram == 1 {
            lines.insert(Line::Tram, true);
        }
        if query.Victoria == 1 {
            lines.insert(Line::Victoria, true);
        }
        return Ok(LinesDB {
            phone_number: phone_number.clone(),
            map: lines,
        });
    }
    async fn remove_from_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM tube_lines WHERE phone_number = ?"#,
            self.phone_number
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
impl DaysDB {
    fn new(phone_number: String, days: Option<impl IntoIterator<Item = Day>>) -> DaysDB {
        let mut map = HashMap::with_capacity(7);
        match days {
            Some(days) => {
                for day in days {
                    map.insert(day, true);
                }
            }
            None => {}
        };
        return DaysDB { phone_number, map };
    }
    async fn insert_into_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let mut query = String::from("INSERT INTO days (phone_number");
        let mut values = format!("VALUES ('{}'", self.phone_number);
        for (day, to_update) in self.map.iter() {
            if *to_update {
                query.push_str(&format!(",{}", day.to_string()));
                values.push_str(", TRUE ");
            } else {
                query.push_str(&format!(",{}", day.to_string()));
                values.push_str(", FALSE ");
            }
        }
        query.push_str(") ");
        values.push_str(");");
        query.push_str(&values);
        sqlx::query(&query).execute(pool).await?;
        Ok(())
    }
    pub async fn fetch(phone_number: &String, pool: &MySqlPool) -> Result<DaysDB, sqlx::Error> {
        let mut days = HashMap::with_capacity(7);
        let db_query = sqlx::query!("SELECT * FROM days WHERE phone_number = ?", phone_number)
            .fetch_one(pool)
            .await?;
        if db_query.Monday == 1 {
            days.insert(Day::Monday, true);
        }
        if db_query.Tuesday == 1 {
            days.insert(Day::Tuesday, true);
        }
        if db_query.Wednesday == 1 {
            days.insert(Day::Wednesday, true);
        }
        if db_query.Thursday == 1 {
            days.insert(Day::Thursday, true);
        }
        if db_query.Friday == 1 {
            days.insert(Day::Friday, true);
        }
        if db_query.Saturday == 1 {
            days.insert(Day::Saturday, true);
        }
        if db_query.Sunday == 1 {
            days.insert(Day::Sunday, true);
        }
        return Ok(DaysDB {
            phone_number: phone_number.clone(),
            map: days,
        });
    }
    async fn remove_from_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM days WHERE phone_number = ?", self.phone_number)
            .execute(pool)
            .await?;
        Ok(())
    }
}
impl User {
    fn new(phone_number: String, update_time: NaiveTime) -> User {
        User {
            phone_number,
            update_time,
        }
    }
    async fn insert_into_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (phone_number, update_time) VALUES (?, ?)",
            self.phone_number,
            self.update_time,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    async fn fetch(phone_number: String, pool: &MySqlPool) -> Result<User, sqlx::Error> {
        let time = sqlx::query!(
            "SELECT update_time FROM users WHERE phone_number = ?",
            phone_number
        )
        .fetch_one(pool)
        .await?;
        return Ok(User {
            phone_number: phone_number.clone(),
            update_time: time.update_time,
        });
    }
    async fn remove_from_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users WHERE phone_number = ?",
            self.phone_number
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
impl Recipient {
    pub fn new(
        phone_number: String,
        update_time: NaiveTime,
        days: Option<Vec<Day>>,
        lines: Option<Vec<Line>>,
    ) -> Recipient {
        Recipient {
            user: User::new(phone_number.clone(), update_time),
            days: DaysDB::new(phone_number.clone(), days),
            lines: LinesDB::new(phone_number.clone(), lines),
        }
    }
    pub async fn insert_into_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        self.user.insert_into_db(pool).await?;

        let (days, lines) = tokio::join!(
            self.days.insert_into_db(pool),
            self.lines.insert_into_db(pool)
        );
        days?;
        lines?;
        Ok(())
    }
    pub async fn fetch(phone_number: String, pool: &MySqlPool) -> Result<Recipient, sqlx::Error> {
        let user = User::fetch(phone_number.clone(), pool).await?;
        let (days, lines) = tokio::join!(
            DaysDB::fetch(&user.phone_number, pool),
            LinesDB::fetch(&user.phone_number, pool)
        );

        return Ok(Recipient {
            user: User::new(user.phone_number, user.update_time),
            days: days?,
            lines: lines?,
        });
    }
    pub async fn remove_from_db(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let (res_lines, res_days) = tokio::join!(
            self.lines.remove_from_db(pool),
            self.days.remove_from_db(pool)
        );
        res_lines?;
        res_days?;
        self.user.remove_from_db(pool).await?;
        Ok(())
    }
}
#[cfg(test)]
mod dbtests {
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
}
