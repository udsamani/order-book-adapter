use crate::handlers::instruments::get_best_bid;
use crate::handlers::instruments::get_order_book;
use crate::handlers::instruments::subscribe_instrument;
use crate::handlers::instruments::unsubscribe_instrument;
use crate::AppState;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

pub fn oba_router(state: AppState) -> Router {
    Router::new().nest("/api/v1/instruments", instrument_routes(state.clone()))
}

fn instrument_routes(state: AppState) -> Router {
    Router::new()
        .route("/subscribe", post(subscribe_instrument))
        .route("/unsubscribe", post(unsubscribe_instrument))
        .route("/:name/orderbook", get(get_order_book))
        .route("/:name/bestbid", get(get_best_bid))
        .with_state(state)
}
