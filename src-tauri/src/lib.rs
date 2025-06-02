use tauri::Manager;

mod db;
mod file;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let app_data_dir = app.path().app_data_dir().unwrap();

                let db_state = db::database::DbState {
                    pool: db::database::initialise_db(app_data_dir).await.unwrap()
                };

                app.manage(db_state)
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            db::location::get_all_locations,
            db::location::add_location,
            db::location::delete_location,
            db::track::get_all_tracks,
            db::track::delete_track,
            db::track::delete_multiple_tracks,
            file::import_audio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
