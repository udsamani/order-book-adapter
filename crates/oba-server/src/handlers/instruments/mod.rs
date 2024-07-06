pub use get_order_book::get_order_book;
use serde::Deserialize;
pub use subscribe_instrument::subscribe_instrument;
pub use unsubscribe_instrument::unsubscribe_instrument;

mod get_order_book;
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
