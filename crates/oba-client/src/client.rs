use std::time::Duration;

use url::Url;

use crate::models::{OBAClientError, SubscribeInstrumentRequest, UnsubscribeInstrument};

const OBA_CLIENT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const OBA_SUBSCRIBE_INSTRUMENT_PATH: &str = "/api/v1/instruments/subscribe";
const OBA_UNSUBSCRIBE_INSTRUMENT_PATH: &str = "/api/v1/instruments/unsubscribe";

#[derive(Clone, Debug)]
pub struct OBAClient {
    pub http: reqwest::Client,
    pub endpoint: Url,
}

impl OBAClient {
    pub fn new(http: reqwest::Client, endpoint: Url) -> Self {
        Self { http, endpoint }
    }

    pub fn from_endpoint_str(endpoint: &str) -> Self {
        let endpoint = Url::parse(endpoint).unwrap();
        let client = reqwest::ClientBuilder::new()
            .timeout(OBA_CLIENT_REQUEST_TIMEOUT)
            .build()
            .unwrap();
        Self::new(client, endpoint)
    }


    pub async fn subscribe_instrument(&self, instrument: &str, order_book_depth: usize) -> Result<(), OBAClientError> {
        let request = SubscribeInstrumentRequest{
            name: instrument.to_string(),
            order_book_depth
        };


        let response = self.http
            .post(self.endpoint.join(OBA_SUBSCRIBE_INSTRUMENT_PATH).unwrap())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await;

        match response {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                Err(OBAClientError::from(e))
            }
        }
    }

    pub async fn unsubscribe_instrument(&self, instrument: &str) -> Result<(), OBAClientError> {

        let request = UnsubscribeInstrument{
            name: instrument.to_string()
        };

        let response = self.http
            .post(self.endpoint.join(OBA_UNSUBSCRIBE_INSTRUMENT_PATH).unwrap())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await;

        match response {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                Err(OBAClientError::from(e))
            }
        }
    }

}
