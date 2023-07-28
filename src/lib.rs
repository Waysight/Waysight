use std::{path::PathBuf, sync::Mutex};
pub mod backend;
pub mod config;
pub mod handlers;
pub mod state;
pub mod utils;

pub struct UserData {
    pub config_path: Option<PathBuf>,
}
pub static USER_DATA: Mutex<UserData> = Mutex::new(UserData { config_path: None });
