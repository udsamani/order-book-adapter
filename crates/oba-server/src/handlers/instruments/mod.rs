use serde::Deserialize;
pub use subscribe_instrument::subscribe_instrument;
pub use unsubscribe_instrument::unsubscribe_instrument;

mod subscribe_instrument;
mod unsubscribe_instrument;

#[derive(Debug, Deserialize)]
pub struct SubscribeInstrumentRequest {
    name: String,
    order_book_depth: usize,
}

#[derive(Debug, Deserialize)]
pub struct UnsubscribeInstrumentRequest {
    name: String,
}
