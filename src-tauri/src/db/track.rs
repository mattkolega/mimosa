use super::database::DbState;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tauri::State;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Track {
    id: i32,
    title: String,
    artist: Option<String>,
    album: Option<String>,
    filepath: String,
}

#[tauri::command]
pub async fn get_all_tracks(state: State<'_, DbState>) -> Result<Vec<Track>, String> {
    let result: Vec<Track> = sqlx::query_as(
        "SELECT * FROM track"
    )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| "Failed to get all tracks.".to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn add_track(state: State<'_, DbState>, track: Track) -> Result<i64, String> {
    let result = sqlx::query(
        "INSERT INTO track VALUES ($1, $2, $3, $4)"
    )
        .bind(&track.title)
        .bind(&track.artist)
        .bind(&track.album)
        .bind(&track.filepath)
        .execute(&state.pool)
        .await
        .map_err(|_| "Failed to add track.".to_string())?;

    Ok(result.last_insert_rowid())
}

#[tauri::command]
pub async fn add_multiple_tracks(state: State<'_, DbState>, tracks: Vec<Track>) -> Result<u64, String> {
    let mut tx = state.pool.begin()
        .await
        .map_err(|_| "Failed to start transaction for adding tracks.".to_string())?;

    let mut rows_affected = 0;

    for track in tracks.into_iter() {
        let result = sqlx::query(
            "INSERT INTO track VALUES ($1, $2, $3, $4)"
        )
            .bind(&track.title)
            .bind(&track.artist)
            .bind(&track.album)
            .bind(&track.filepath)
            .execute(&mut *tx)
            .await
            .map_err(|_| "Failed to add tracks.".to_string())?;

        rows_affected += result.rows_affected();
    }

    tx.commit()
        .await
        .map_err(|_| "Failed to commit transaction for adding tracks.".to_string())?;

    Ok(rows_affected)
}

#[tauri::command]
pub async fn delete_track(state: State<'_, DbState>, track_id: i32) -> Result<u64, String> {
    let result = sqlx::query(
        "DELETE FROM track VALUES ($1)"
    )
        .bind(track_id)
        .execute(&state.pool)
        .await
        .map_err(|_| "Failed to delete track.".to_string())?;

    Ok(result.rows_affected())
}

#[tauri::command]
pub async fn delete_multiple_tracks(state: State<'_, DbState>, track_ids: Vec<i32>) -> Result<u64, String> {
    let values = track_ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

    let result = sqlx::query(
        "DELETE FROM track WHERE id IN ($1)"
    )
        .bind(values)
        .execute(&state.pool)
        .await
        .map_err(|_| "Failed to delete tracks.".to_string())?;

    Ok(result.rows_affected())
}
