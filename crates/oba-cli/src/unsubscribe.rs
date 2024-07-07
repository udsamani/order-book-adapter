use oba_client::client::OBAClient;



pub async fn unsubscribe_instrument(instrument: &str) {
    let oba_client = OBAClient::from_endpoint_str("http://localhost:3000");
    oba_client.unsubscribe_instrument(instrument).await;
}
