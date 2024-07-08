use oba_client::client::OBAClient;


pub async fn best_ask(address: &str, instrument: &str) {

    let client = OBAClient::from_endpoint_str(address);
    let response = client.get_best_ask(instrument).await;

    match response {
        Ok(response) => {
            println!("{}", response.best_ask);
        },
        Err(e) => {
            println!("{}", e.to_string());
        }
        
    }
}

