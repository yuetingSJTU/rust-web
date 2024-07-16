use std::sync::Mutex;
use super::models::Hookup;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub hookups: Mutex<Vec<Hookup>>,
}
