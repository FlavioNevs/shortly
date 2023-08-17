use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};


#[derive(Serialize, sqlx::FromRow)]
pub struct Url {
    pub id: String,
    pub long_url: String
}

impl From<NewUrl> for Url {
    fn from(value: NewUrl) -> Self {
        let id = &general_purpose::URL_SAFE_NO_PAD.encode(&value.long_url)[..7];

        Self {
            id: String::from(id),
            long_url: value.long_url
        }
    }
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct NewUrl {
    pub long_url: String
}