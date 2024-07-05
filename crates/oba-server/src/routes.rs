use axum::routing::post;
use axum::{routing::get, Router};
use crate::handlers::instruments::subscribe_instrument;
use crate::handlers::instruments::unsubscribe_instrument;

pub fn oba_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .nest("/api/v1/instruments", instrument_routes())
}

async fn hello_world() -> String {
    String::from("Hello World")
}


fn instrument_routes() -> Router<> {

    Router::new()
        .route("/subscribe", post(subscribe_instrument))
        .route("/unsubscribe", post(unsubscribe_instrument))
    
}
