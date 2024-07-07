use oba_client::client::OBAClient;



pub async fn subscribe_instrument(instrument: &str, depth: &usize) {
    let oba_client = OBAClient::from_endpoint_str("http://localhost:3000");
    oba_client.subscribe_instrument(instrument, depth.clone()).await;
}
