use axum::http::StatusCode;


use axum::extract::Path;
use axum::response::Redirect;
use axum::{Extension, Json};

use sqlx::SqlitePool;


use crate::models::url;
use crate::errors::Error;


pub async fn get_url(
    Path(id):Path<String>,
    Extension(pool): Extension<SqlitePool>
) -> Result<Redirect, Error> {

    let query = "SELECT * FROM url WHERE id = ?".to_string();
    
    let result = sqlx::query_as::<_, url::Url>(&query)
        .bind(id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(obj) => Ok(Redirect::to(&obj.long_url)),
        Err(_) => Err(Error::UrlNotFound)
    }
}

pub async fn create_short_url(
    Extension(pool): Extension<SqlitePool>,
    Json(new_url): Json<url::NewUrl>
) -> Result<(StatusCode, Json<url::Url>), Error> {

    let obj = url::Url::from(new_url);

    let query = "INSERT INTO url(id, long_url) VALUES (?, ?)".to_string();
    let _ = sqlx::query(&query)
        .bind(&obj.id)
        .bind(&obj.long_url)
        .execute(&pool)
        .await
        .map_err( |_| {
            Error::InternalServerError
        });
    
    Ok((StatusCode::CREATED, Json(obj)))
}
