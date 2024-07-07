use oba_client::client::OBAClient;

pub async fn unsubscribe_instrument(address: &str, instrument: &str) {
    let oba_client = OBAClient::from_endpoint_str(address);
    oba_client.unsubscribe_instrument(instrument).await;
}
