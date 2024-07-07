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

#[derive(Serialize, Default, Debug, Clone)]
pub struct SubscribeInstrumentRequest {
    pub name: String,
    pub order_book_depth: usize,
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct UnsubscribeInstrument {
    pub name: String,
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
}
