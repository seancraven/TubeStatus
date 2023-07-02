use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::Path;

/// Stuct for a user that can be stored as a json.
#[derive(Debug, Serialize, Deserialize, PartialEq, Hash)]
pub struct User {
    pub phone: String,
    pub days: Vec<Days>,
    pub time: GMTHour,
}
impl User {
    /// Find a user by their phone number.
    pub fn find_user(tel_number: &str) -> Option<User> {
        if let Ok(users) = User::get_users() {
            for user in users {
                if user.phone == tel_number {
                    return Some(user);
                }
            }
        }
        return None;
    }
    fn get_users() -> Result<Vec<User>, UserReadError> {
        if !Path::new("users.json").exists() {
            return Err(UserReadError::FileNotFound);
        }
        let string = fs::read_to_string("users.json").map_err(|_| UserReadError::FileReadError)?;
        let users: Vec<User> =
            serde_json::from_str(&string).map_err(|_| UserReadError::JsonError)?;
        return Ok(users);
    }
}

#[derive(Deserialize, Debug)]
enum UserReadError {
    FileNotFound,
    FileReadError,
    JsonError,
}
impl Display for UserReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserReadError::FileNotFound => write!(f, "File not found"),
            UserReadError::FileReadError => write!(f, "File read error"),
            UserReadError::JsonError => write!(f, "Json error"),
        }
    }
}
impl Error for UserReadError {}

/// Serializable struct for time that can be made into a NaiveTime.
#[derive(Debug, Serialize, Deserialize, PartialEq, Hash)]
pub struct GMTHour {
    hour: u8,
    minute: u8,
}
impl GMTHour {
    fn into_naive_opt(self) -> Option<NaiveTime> {
        NaiveTime::from_hms_opt(self.hour.into(), self.minute.into(), 0)
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Hash)]
pub enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use serde_json;
    use std::fs;
    /// Test writing to users.json
    #[test]
    fn test_user() {
        let user = User {
            phone: String::from("1234567890"),
            days: vec![Days::Monday, Days::Tuesday],
            time: GMTHour {
                hour: 12,
                minute: 0,
            },
        };
        let serialized = serde_json::to_string(&user).unwrap();
        fs::write("users.json", serialized).expect("Unable to write file");
        let string_to_deserialise = fs::read_to_string("users.json").expect("Unable to read file");
        let deserialized: User = serde_json::from_str(&string_to_deserialise).unwrap();
        assert_eq!(deserialized, user)
    }
    #[test]
    fn multi_user() {
        let mut rng = rand::thread_rng();

        let base: u64 = 10;
        let upper: u64 = base.pow(12);
        let lower: u64 = base.pow(11);
        let mut num: u64;
        let mut user: User;
        let mut users: Vec<User> = Vec::new();
        for _ in 0..1000 {
            num = rng.gen_range(lower..upper);
            user = User {
                phone: num.to_string(),
                days: vec![Days::Monday, Days::Tuesday],
                time: GMTHour {
                    hour: rng.gen_range(0..24),
                    minute: rng.gen_range(0..60),
                },
            };
            users.push(user);
        }
        let serialized = serde_json::to_string(&users).unwrap();
        fs::write("users.json", serialized).expect("Unable to write file");
        let string_to_deserialise = fs::read_to_string("users.json").expect("Unable to read file");
        let deserialized: Vec<User> = serde_json::from_str(&string_to_deserialise).unwrap();
        for user in deserialized {
            assert_eq!(user, User::find_user(&user.phone).unwrap());
        }
        fs::remove_file("users.json").expect("Unable to remove file");
    }
}
