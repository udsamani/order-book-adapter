use std::collections::BTreeMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum OBAClientError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("URL parse wrror: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Error from API: {0}")]
    Api(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
#[serde(untagged)]
pub enum ApiResult<T: Serialize + DeserializeOwned> {
    Ok(T),
    Err(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
pub struct OrderBookAdapterResponse<T: Serialize + DeserializeOwned> {
    pub data: T,
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct SubscribeInstrumentRequest {
    pub name: String,
    pub order_book_depth: usize,
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct UnsubscribeInstrumentRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct GetOrderBookResponse {
    pub symbol: String,
    pub bids: BTreeMap<u64, f64>,
    pub asks: BTreeMap<u64, f64>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_subscribe_instrument_request_serialization() {
        let expected_request = r#"{"name":"btcusd","order_book_depth":5}"#;

        let request = SubscribeInstrumentRequest {
            name: String::from("btcusd"),
            order_book_depth: 5,
        };

        assert_eq!(expected_request, serde_json::to_string(&request).unwrap());
    }

    #[test]
    fn test_unsubscribe_instrument_request_serialization() {
        let expected_request = r#"{"name":"btcusd"}"#;

        let request = UnsubscribeInstrumentRequest {
            name: String::from("btcusd"),
        };

        assert_eq!(expected_request, serde_json::to_string(&request).unwrap());
    }

    #[test]
    fn test_get_order_book_response() {
        let get_order_book_response = r#"
            {
                "symbol": "btcusd",
                "bids": {
                    "56423": 0.00040883,
                    "56981": 2.84850366,
                    "56982": 0.47928454,
                    "56983": 2.89559645,
                    "56985": 0.21057825
                },
                "asks": {
                    "56922": 0.89596637,
                    "56923": 0.96718099,
                    "56924": 1.0437901900000002,
                    "56925": 4.650952300000002,
                    "57801": 0.00045374
                }
            }
        "#;

        let response =
            serde_json::from_str::<GetOrderBookResponse>(&get_order_book_response).unwrap();
        println!("{:?}", response);
    }
}
