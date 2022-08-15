pub mod credits;
pub mod validate;

use crate::credits::ResponseSuccess as CreditsResponseSuccess;
use crate::validate::ResponseSuccess as ValidateResponseSuccess;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Debug;
use std::net::IpAddr;

pub(crate) const API_URL: &str = "https://api.zerobounce.net/v2";

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseType<T> {
    // When the API responds with success. The HTTP status is 200
    Success(Box<T>),
    // When the API responds with error. The HTTP status is 200
    Error(ResponseError),
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    pub error: String,
}

pub struct Api {
    api_url: String,
    api_key: String,
    client: Client,
}

impl Api {
    pub fn new(api_key: impl Into<String>) -> Api {
        Api {
            api_url: API_URL.to_string(),
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    pub fn set_api_url(&mut self, api_url: impl Into<String>) -> &mut Self {
        self.api_url = api_url.into().trim_end_matches('/').to_string();
        self
    }

    pub fn get_api_url(&self) -> String {
        self.api_url.to_string()
    }

    pub fn set_api_key(&mut self, api_key: impl Into<String>) -> &mut Self {
        self.api_key = api_key.into();
        self
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.to_string()
    }

    pub async fn validate(
        &self,
        email: impl Into<String>,
        ip_address: Option<IpAddr>,
    ) -> Result<ResponseType<ValidateResponseSuccess>, Box<dyn Error>> {
        let email_address = email.into();
        let mut url = format!(
            "{}/validate?api_key={}&email={}",
            self.api_url, self.api_key, email_address
        );
        if ip_address.is_some() {
            url = format!("{}&ip_address={}", url, ip_address.unwrap());
        }

        let response: ResponseType<ValidateResponseSuccess> = self
            .client
            .get(url)
            .send()
            .await?
            .json::<ResponseType<ValidateResponseSuccess>>()
            .await?;

        Ok(response)
    }

    pub async fn get_credits(
        &self,
    ) -> Result<ResponseType<CreditsResponseSuccess>, Box<dyn Error>> {
        let url: String = format!("{}/getcredits?api_key={}", self.api_url, self.api_key);

        let response: ResponseType<CreditsResponseSuccess> = self
            .client
            .get(url)
            .send()
            .await?
            .json::<ResponseType<CreditsResponseSuccess>>()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::Api;

    #[test]
    fn api_url() {
        let check_api_url: &str = "https://example.com";

        let mut api: Api = Api::new("test");

        assert_eq!(crate::API_URL.to_string(), api.get_api_url());

        api.set_api_url(check_api_url);
        assert_eq!(check_api_url.to_string(), api.get_api_url());
    }

    #[test]
    fn api_key() {
        let mut api: Api = Api::new("test");

        assert_eq!("test".to_string(), api.get_api_key());

        api.set_api_key("test 2");
        assert_eq!("test 2".to_string(), api.get_api_key());
    }
}
