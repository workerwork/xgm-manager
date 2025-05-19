use crate::models::auth::LoginResponse;
use crate::services::auth_service;

#[tauri::command]
pub fn login(username: String, password: String) -> Result<LoginResponse, String> {
    auth_service::login(&username, &password)
}
