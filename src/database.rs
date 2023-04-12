use chrono::naive::NaiveTime;
use sqlx::mysql::{MySqlPool, MySqlQueryResult};
// TODO, fix the add user tabel,
// TODO, fix the get user tabel,
// TODO, restructure the database so that the user tabel's pk is
// the phone number, use this as the fk in the other tabels.
#[derive(Debug, PartialEq, Eq)]
struct UserTabel {
    id: u8,
    name: String,
    phone_number: String,
    update_time: NaiveTime,
    days: DaysTabel,
    lines: LinesTabel,
}
impl UserTabel {
    pub fn new(
        id: u8,
        name: String,
        phone_number: String,
        update_time: NaiveTime,
        days: impl IntoIterator<Item = bool>,
        lines: impl IntoIterator<Item = bool>,
    ) -> Self {
        let days = DaysTabel::new(days).unwrap();
        let lines = LinesTabel::new(lines).unwrap();
        UserTabel {
            id,
            name,
            phone_number,
            update_time,
            days,
            lines,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
enum CreateDaysError {
    IncorrectDayCount,
    InvalidDay,
}
#[derive(Debug, PartialEq, Eq)]
struct DaysTabel {
    days: Vec<bool>,
}
impl DaysTabel {
    pub fn new(iterable: impl IntoIterator<Item = bool>) -> Result<Self, CreateDaysError> {
        let mut days = Vec::with_capacity(7);
        for day in iterable.into_iter() {
            days.push(day);
        }
        if days.len() != 7 {
            return Err(CreateDaysError::IncorrectDayCount);
        }
        return Ok(DaysTabel { days });
    }
    pub fn from_strs<'a>(
        iterable: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, CreateDaysError> {
        let mut days = vec![false; 7];
        for day in iterable.into_iter() {
            match day {
                "monday" => days[0] = true,
                "tuesday" => days[1] = true,
                "wednesday" => days[2] = true,
                "thursday" => days[3] = true,
                "friday" => days[4] = true,
                "saturday" => days[5] = true,
                "sunday" => days[6] = true,
                _ => return Err(CreateDaysError::InvalidDay),
            }
        }
        return Ok(DaysTabel { days });
    }
}

/// Struct to match the database tabel, for ease
/// of use.
#[derive(Debug, PartialEq, Eq)]
struct LinesTabel {
    lines: Vec<bool>,
    // elizabeth: bool,
    // jubilee: bool,
    // bakerloo: bool,
    // central: bool,
    // circle: bool,
    // district: bool,
    // dlr: bool,
    // hammersmith: bool,
    // metropolitan: bool,
    // northern: bool,
    // piccadilly: bool,
    // victoria: bool,
    // waterloo: bool,
    // overground: bool,
    // tram: bool,
}
#[derive(Debug, PartialEq, Eq)]
enum CreateLinesError {
    IncorrectLineCount,
    InvalidLine,
}
impl LinesTabel {
    pub fn new(iterable: impl IntoIterator<Item = bool>) -> Result<Self, CreateLinesError> {
        let mut lines = Vec::with_capacity(15);
        for line in iterable.into_iter() {
            lines.push(line);
        }
        if lines.len() != 15 {
            return Err(CreateLinesError::IncorrectLineCount);
        }
        return Ok(LinesTabel { lines });
    }
    pub fn from_strs<'a>(
        iterable: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, CreateLinesError> {
        let mut lines = vec![false; 15];

        for line in iterable.into_iter() {
            match line {
                "elizabeth" => lines[0] = true,
                "jubilee" => lines[1] = true,
                "bakerloo" => lines[2] = true,
                "central" => lines[3] = true,
                "circle" => lines[4] = true,
                "district" => lines[5] = true,
                "dlr" => lines[6] = true,
                "hammersmith" => lines[7] = true,
                "metropolitan" => lines[8] = true,
                "northern" => lines[9] = true,
                "piccadilly" => lines[10] = true,
                "victoria" => lines[11] = true,
                "waterloo" => lines[12] = true,
                "overground" => lines[13] = true,
                "tram" => lines[14] = true,
                _ => return Err(CreateLinesError::InvalidLine),
            }
        }
        return Ok(LinesTabel { lines });
    }
}
/// Make the tabels if they don't exist.
async fn create_tabels(conn: &MySqlPool) {
    let users = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SMALLINT NOT NULL AUTO_INCREMENT,
            name VARCHAR(20) NOT NULL,
            phone_number VARCHAR(255) NOT NULL UNIQUE,
            update_time TIME NOT NULL,
            CONSTRAINT pk_user PRIMARY KEY (id)
        )
        "#,
    )
    .execute(conn);

    let days = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS days (
            id SMALLINT NOT NULL AUTO_INCREMENT,
            mon BOOLEAN NOT NULL,
            tue BOOLEAN NOT NULL,
            wed BOOLEAN NOT NULL,
            thu BOOLEAN NOT NULL,
            fri BOOLEAN NOT NULL,
            sat BOOLEAN NOT NULL,
            sun BOOLEAN NOT NULL,
            CONSTRAINT fk_days FOREIGN KEY (id) REFERENCES users(id)
        )
        "#,
    )
    .execute(conn);

    let lines = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS tube_lines (
            id SMALLINT NOT NULL AUTO_INCREMENT,
            elizabeth BOOLEAN NOT NULL,
            jubilee BOOLEAN NOT NULL,
            bakerloo BOOLEAN NOT NULL,
            central BOOLEAN NOT NULL,
            circle BOOLEAN NOT NULL,
            district BOOLEAN NOT NULL,
            dlr BOOLEAN NOT NULL,
            hammersmith BOOLEAN NOT NULL,
            metropolitan BOOLEAN NOT NULL,
            northern BOOLEAN NOT NULL,
            piccadilly BOOLEAN NOT NULL,
            victoria BOOLEAN NOT NULL,
            waterloo BOOLEAN NOT NULL,
            overground BOOLEAN NOT NULL,
            tram BOOLEAN NOT NULL,
            CONSTRAINT fk_lines FOREIGN KEY (id) REFERENCES users(id)
        )
        "#,
    )
    .execute(conn);

    let (r_1, r_2, r_3) = tokio::join!(users, days, lines);
    r_1.expect("Failed to create users table");
    r_2.expect("Failed to create days table");
    r_3.expect("Failed to create lines table");
}

///  Add a user to the database.
async fn add_user(
    pool: &MySqlPool,
    user_tabel: &UserTabel,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (name, phone_number, update_time)
        VALUES (?, ?, ?)
        "#,
        user_tabel.name,
        user_tabel.phone_number,
        user_tabel.update_time
    )
    .execute(pool)
    .await
}
/// Get a user from the database, via their phone number, this
/// is the natural key for the user.
///
///
/// # Errors
/// If the user does not exist, or the database is down, this will
/// return an error.
///
/// # Examples
/// ```
/// use database::get_user;
/// use sqlx::mysql::MySqlPool;
/// use std::env;
/// use dotenv::dotenv;
/// dotenv().ok();
/// // set some database url.
/// let pool = MySqlPool::connect(&db_url)
///    .await
///    .expect("Failed to connect to database");
/// let user = get_user(&pool, &String::from("07777777777")).await;
///
///
async fn get_user(pool: &MySqlPool, number: &String) -> Result<UserTabel, sqlx::Error> {
    let user = sqlx::query!(
        r#"
        SELECT * FROM users WHERE phone_number=?;
        "#,
        number
    )
    .fetch_one(pool)
    .await?;
    todo!();
    // Ok(UserTabel {
    //     id: user.id as u8,
    //     name: user.name,
    //     phone_number: user.phone_number,
    //     update_time: user.update_time,
    // })
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPool;
    use std::env;

    #[tokio::test]
    async fn create_tabels_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPool::connect(&db_url)
            .await
            .expect("Failed to connect to database");
        create_tabels(&pool).await;
        assert!(true);
    }
    #[tokio::test]
    async fn user_add_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPool::connect(&db_url)
            .await
            .expect("Failed to connect to database");
        let dave = UserTabel {
            id: 0,
            name: "Test".to_string(),
            phone_number: "123456789".to_string(),
            update_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            days: DaysTabel {
                days: vec![false; 7],
            },
            lines: LinesTabel {
                lines: vec![false; 15],
            },
        };
        sqlx::query!("DELETE FROM users WHERE name='Test';")
            .execute(&pool)
            .await
            .expect("Failed to delete test user");

        add_user(&pool, &dave).await.expect("Failed to add user");
        assert!(true);
    }
    #[tokio::test]
    async fn add_remove_user_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPool::connect(&db_url)
            .await
            .expect("Failed to connect to database");
        let dave = UserTabel {
            id: 0,
            name: "Test".to_string(),
            phone_number: "123456789".to_string(),
            update_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            days: DaysTabel {
                days: vec![false; 7],
            },
            lines: LinesTabel {
                lines: vec![false; 15],
            },
        };
        sqlx::query!("DELETE FROM users WHERE name='Test';")
            .execute(&pool)
            .await
            .expect("Failed to delete test user");

        add_user(&pool, &dave).await.expect("Failed to add user");
        let user = get_user(&pool, &String::from("123456789"))
            .await
            .expect("Failed to get user");
        assert_eq!(user.name, dave.name);
        assert_eq!(user.phone_number, dave.phone_number);
    }
}
