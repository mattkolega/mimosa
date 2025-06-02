use async_recursion::async_recursion;
use lofty::file::TaggedFileExt;
use lofty::tag::Accessor;
use std::path::{Path, PathBuf};
use tokio::{fs};
use tauri::State;

use crate::db::database::DbState;
use crate::db::track;

/// Scans a directory for audio files and loads them into the database
#[tauri::command]
pub async fn import_audio (state: State<'_, DbState>, dir: String) -> Result<u32, String> {
    let filepaths = scan_dir(PathBuf::from(dir)).await?;

    let mut track_list = vec![];

    for filepath in filepaths.into_iter() {
        let track = get_initial_track_info(&filepath).await?;
        track_list.push(track);
    }

    track::add_multiple_tracks(state, track_list).await?;

    Ok(0)
}

/// Recursively searches a directory for audio files and returns them as a list of paths
#[async_recursion]
async fn scan_dir(dir: PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut entries = fs::read_dir(dir.as_path())
        .await
        .map_err(|err| format!(
            "Failed to read directory '{}' while scanning for audio files: {}",
            dir.display(),
            err
        ))?;

    let mut audio_files = vec![];

    while let Some(entry) = entries.next_entry().await.map_err(|err| format!("Failed to grab next entry from dir '{}': {}", dir.display(), err))? {
        let file_type = entry.file_type().await
            .map_err(|err| format!("Failed to get file type for file '{}': {}", entry.path().display(), err))?;

        if file_type.is_dir() {
            let mut child_dir_files = scan_dir(entry.path()).await?;
            audio_files.append(&mut child_dir_files);
            continue;
        }
        if !file_type.is_file() { continue; }

        // We only want files which are MP3's,sor FLAC's or WAV's
        let filepath = entry.path();
        let extension = filepath.extension().and_then(|ext| ext.to_str());
        if let Some(ext) = extension {
            match ext.to_ascii_lowercase().as_str() {
                "mp3" | "flac" | "wav" => { audio_files.push(filepath); }
                _ => ()
            }
        }
    }

    Ok(audio_files)
}

async fn get_initial_track_info(filepath: &Path) -> Result<track::Track, String> {
    let filepath = filepath.to_path_buf();

    tauri::async_runtime::spawn_blocking(move || {
        let tagged_file = lofty::read_from_path(&filepath)
            .map_err(|err| format!("Failed to read file '{}' for metadata: {}", filepath.display(), err))?;

        if let Some(tag) = tagged_file.primary_tag() {
            let title = tag.title()
                .map(|value| value.to_string())
                .or_else(|| filepath.file_name().map(|value| value.to_string_lossy().to_string()));
            let artist = tag.artist().map(|value| value.to_string());
            let album = tag.album().map(|value| value.to_string());

            Ok(track::Track {
                id: None,
                title,
                artist,
                album,
                filepath: filepath.to_string_lossy().to_string(),
            })
        } else {
            Err(format!("Failed to access primary tag for music file: '{}'", filepath.display()))
        }
    }).await
    .map_err(|err| format!("Failed to create task for grabbing track metadata: {}", err))?
}