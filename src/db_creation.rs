use dotenv::dotenv;
use std::env;
#[tokio::main]
fn main() {
    dotenv().ok();
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").expect("Expected to get database url"))
        .await
        .expect("Failed to connect to database");
    create_tables(&pool).await;
}
async fn create_tables(conn: &MySqlPool) {
    let users = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            phone_number VARCHAR(20) NOT NULL UNIQUE,
            update_time TIME NOT NULL,
            CONSTRAINT pk_user PRIMARY KEY (phone_number)
        )
        "#,
    )
    .execute(conn);

    let days = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS days (
            phone_number VARCHAR(20) NOT NULL UNIQUE,
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
            phone_number VARCHAR(20) NOT NULL UNIQUE,
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
mod db_creation_tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn crate_tabel_test() {
        dotenv().ok();
        let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        create_tables(&pool).await;
    }
}
