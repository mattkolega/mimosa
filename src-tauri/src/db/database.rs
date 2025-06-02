use std::error::Error;
use std::path::Path;

use sqlx::{migrate::MigrateDatabase, sqlite, Pool, Sqlite};
use tokio::fs::create_dir_all;

pub struct DbState {
    pub pool: Pool<Sqlite>
}

pub async fn initialise_db(path: &Path) -> Result<Pool<Sqlite>, Box<dyn Error>> {
    create_dir_all(path).await?;

    let path_str = path.to_str().ok_or("Path is not valid UTF-8")?;
    let conn_url = format!("sqlite://{}/{}", path_str, "mimosa.db");

    if !Sqlite::database_exists(&conn_url).await.unwrap_or(false) {
        Sqlite::create_database(&conn_url).await?;
    }

    let pool = sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&conn_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS track (
            id INTEGER PRIMARY KEY,
            title TEXT,
            artist TEXT,
            album TEXT,
            filepath TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS location (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL
        );"
    )
        .execute(&pool)
        .await?;

    Ok(pool)
}