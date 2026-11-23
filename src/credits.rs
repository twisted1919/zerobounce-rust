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
