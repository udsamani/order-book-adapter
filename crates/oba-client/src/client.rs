use std::time::Duration;

use url::Url;

use crate::models::{ApiResult, BestAskResponse, BestBidResponse, GetOrderBookResponse, OBAClientError, OrderBookAdapterResponse, SubscribeInstrumentRequest, UnsubscribeInstrumentRequest};

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

        let request = UnsubscribeInstrumentRequest{
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

    pub async fn get_order_book(&self, instrument: &str) -> Result<GetOrderBookResponse, OBAClientError> {

        let path = format!("/api/v1/instruments/{}/orderbook", instrument);

        let result= self.http
            .get(self.endpoint.join(&path).unwrap())
            .header("Accept", "application/json")
            .send()
            .await;


        match result {
            Ok(result) => {
                if result.status().is_success() {
                    Ok(result.json().await?)
                } else {
                    Err(OBAClientError::Api(result.text().await?))
                }
            }
            Err(e) => {
                Err(OBAClientError::Api(e.to_string()))
            }
        }


    }

    pub async fn get_best_bid(&self, instrument: &str) -> Result<BestBidResponse, OBAClientError> {

        let path = format!("/api/v1/instruments/{}/bestbid", instrument);

        let result= self.http
            .get(self.endpoint.join(&path).unwrap())
            .header("Accept", "application/json")
            .send()
            .await;


        match result {
            Ok(result) => {
                if result.status().is_success() {
                    Ok(result.json().await?)
                } else {
                    Err(OBAClientError::Api(result.text().await?))
                }
            }
            Err(e) => {
                Err(OBAClientError::Api(e.to_string()))
            }
        }


    }

    pub async fn get_best_ask(&self, instrument: &str) -> Result<BestAskResponse, OBAClientError> {

        let path = format!("/api/v1/instruments/{}/bestask", instrument);

        let result= self.http
            .get(self.endpoint.join(&path).unwrap())
            .header("Accept", "application/json")
            .send()
            .await;


        match result {
            Ok(result) => {
                if result.status().is_success() {
                    Ok(result.json().await?)
                } else {
                    Err(OBAClientError::Api(result.text().await?))
                }
            }
            Err(e) => {
                Err(OBAClientError::Api(e.to_string()))
            }
        }


    }

    

}
