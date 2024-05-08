use actix_web::{post, web, HttpResponse};
use sqlx::{types::chrono::Utc, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// Actix-web makes a hash map of types in its app state, then checks the argument type generic
// passed in here to fetch the proper state. Does this mean you can only have one of each type of
// data in the app state?
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, db_pool),
    fields(
        subscriber_email= %form.email,
        subscriber_name = %form.name
    )
)]
#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&db_pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database.",
    skip(form, db_pool)
)]
pub async fn insert_subscriber(db_pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#" INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4) "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
