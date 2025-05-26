use super::database::DbState;

use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use tauri::State;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Location {
    id: i32,
    path: String
}

#[tauri::command]
pub async fn get_all_locations(state: State<'_, DbState>) -> Result<Vec<Location>, String> {
    let result: Vec<Location> = sqlx::query_as(
        "SELECT * FROM location"
    )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| "Failed to grab music locations.".to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn add_location(state: State<'_, DbState>, path: String) -> Result<i64, String> {
    let result = sqlx::query(
        "INSERT INTO location (path) VALUES ($1)"
    )
        .bind(path)
        .execute(&state.pool)
        .await
        .map_err(|_| "Failed to add music location.".to_string())?;

    Ok(result.last_insert_rowid())
}

#[tauri::command]
pub async fn delete_location(state: State<'_, DbState>, path: String) -> Result<u64, String> {
    let result = sqlx::query(
        "DELETE FROM location WHERE path = $1"
    )
        .bind(path)
        .execute(&state.pool)
        .await
        .map_err(|_| "Failed to delete music location.".to_string())?;

    Ok(result.rows_affected())
}