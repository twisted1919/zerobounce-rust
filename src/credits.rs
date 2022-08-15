use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct ResponseSuccess {
    // the return value is actually a number inside double quotes, so a string...
    #[serde(rename = "Credits")]
    credits: String,
}

impl ResponseSuccess {
    pub fn get_credits(&self) -> u32 {
        self.credits.parse::<u32>().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Api, ResponseType};
    use std::env;

    #[tokio::test]
    async fn get_credits() {
        let api_key: String =
            env::var("ZEROBOUNCE_API_KEY").expect("$ZEROBOUNCE_API_KEY is not set");
        let api: Api = Api::new(api_key);

        let result = api.get_credits().await;

        let mut credits_count: i32 = -1;
        let mut error_message: String = String::from("");
        if let Ok(ResponseType::Success(success)) = result {
            credits_count = success.get_credits() as i32;
        } else if let Ok(ResponseType::Error(error)) = result {
            error_message = error.error;
        }

        assert!(error_message.is_empty(), "{}", error_message);
        assert!(credits_count >= 0);
    }
}
