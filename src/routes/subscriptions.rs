use actix_web::{post, web, HttpResponse};
use sqlx::{types::chrono::Utc, PgPool};
use tracing_futures::Instrument;
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
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email= %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();

    // Don't need to call .enter() as the instrument takes care of it for us.
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#" INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4) "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscribed details have been saved.",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}.",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
    // _request_span_guard is dropped at end of scope.
}
