use axum::routing::post;
use axum::Router;
use crate::handlers::instruments::subscribe_instrument;
use crate::handlers::instruments::unsubscribe_instrument;
use crate::AppState;

pub fn oba_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/instruments", instrument_routes(state.clone()))
}

async fn hello_world() -> String {
    String::from("Hello World")
}


fn instrument_routes(state: AppState) -> Router {

    Router::new()
        .route("/subscribe", post(subscribe_instrument))
        .route("/unsubscribe", post(unsubscribe_instrument))
        .with_state(state)
    
}
