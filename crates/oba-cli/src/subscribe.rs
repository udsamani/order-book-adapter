use oba_client::client::OBAClient;



pub async fn subscribe_instrument(address: &str, instrument: &str, depth: &usize) {
    let oba_client = OBAClient::from_endpoint_str(address);
    oba_client.subscribe_instrument(instrument, depth.clone()).await;
}
