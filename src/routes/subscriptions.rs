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
#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#" INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4) "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
