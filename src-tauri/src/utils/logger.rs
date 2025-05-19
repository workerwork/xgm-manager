use log::LevelFilter;
use tauri::App;
use tauri_plugin_log::Builder as LogBuilder;

pub fn setup_log_plugin(app: &App) -> Result<(), tauri::Error> {
    if cfg!(debug_assertions) {
        app.handle()
            .plugin(LogBuilder::default().level(LevelFilter::Info).build())?;
    }
    Ok(())
}
