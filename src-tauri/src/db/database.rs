use std::error::Error;
use std::path::PathBuf;

use sqlx::{sqlite, Pool, Sqlite};

pub struct DbState {
    pub pool: Pool<Sqlite>
}

pub async fn initialise_db(path: PathBuf) -> Result<Pool<Sqlite>, Box<dyn Error>> {
    let path_str = path.to_str().ok_or("Path is not valid UTF-8")?;
    let db_url = format!("sqlite://{}/{}", path_str, "mimosa.db");

    let pool = sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS track (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
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