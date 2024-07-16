use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hookup {
    pub friend_id: usize,
    pub id: Option<usize>,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

impl From<web::Json<Hookup>> for Hookup {
    fn from(hookup: web::Json<Hookup>) -> Self {
        Hookup {
            friend_id: hookup.friend_id,
            id: hookup.id,
            name: hookup.name.clone(),
            time: hookup.time,
        }
    }
}