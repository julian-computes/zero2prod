use axum::extract::rejection::FormRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SubscribeForm {
    name: String,
    email: String,
}

pub async fn subscribe(
    db_pool: State<Arc<PgPool>>,
    form: Result<Form<SubscribeForm>, FormRejection>,
) -> impl IntoResponse {
    match form {
        Ok(form) => {
            match sqlx::query!(
                r#"
                INSERT INTO subscriptions (email, name, subscribed_at)
                VALUES ($1, $2, $3)
                "#,
                form.email,
                form.name,
                Utc::now()
            )
            .execute(db_pool.0.as_ref())
            .await
            {
                Ok(_) => StatusCode::OK,
                Err(e) => {
                    eprintln!("Failed to execute query: {e}");
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
