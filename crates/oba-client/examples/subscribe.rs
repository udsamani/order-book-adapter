use url::Url;

extern crate oba_client;
extern crate reqwest;
extern crate url;



#[tokio::main]
async fn main() {

    let http_client = reqwest::Client::new();
    let url = Url::parse("http://127.0.0.1:3000").unwrap();
    let client = oba_client::client::OBAClient::new(http_client, url);

    client.subscribe_instrument("btcusd", 5).await;
    
}
