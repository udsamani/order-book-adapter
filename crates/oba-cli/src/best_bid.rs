use oba_client::client::OBAClient;


pub async fn best_bid(address: &str, instrument: &str) {

    let client = OBAClient::from_endpoint_str(address);
    let response = client.get_best_bid(instrument).await;

    match response {
        Ok(response) => {
            println!("{}", response.best_bid);
        },
        Err(e) => {
            println!("{}", e.to_string());
        }
        
    }
}

