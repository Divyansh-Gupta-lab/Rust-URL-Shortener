use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn connect_db(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}
