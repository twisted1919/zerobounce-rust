# ZeroBounce Email Verification Library for Rust

## Getting Started
You will need a [zerobounce account](https://www.zerobounce.net) to get started.  
Once you get an account, you will need to [get an api key](https://www.zerobounce.net/members/apikey/)
to use it in the API calls.

## Installation  
Add below to your `[dependencies]` section of `Cargo.toml`  
`zerobounce = { version = "1.0" }`

## Usage 

```rust 
use std::net::{IpAddr, Ipv4Addr};
use zerobounce::{Api, ResponseType};

// Example function to validate an email address
async fn validate(api: &Api, email: &str) {

    // do the validation using a custom IP Address
    let ip_address = Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    // or use no IP Address
    // let ip_address = None;

    // validate email address
    let result = api.validate(email, ip_address).await;

    match result {
        Err(error) => {
            // this means we got an error during the http call or after
            println!("Error: {}", error);
        },
        Ok(response) => {
            // this means our http call was ok
            match response {
                ResponseType::Success(s) => {
                    // the API call ran okay, we have a response
                    println!("This email is: {}", s.status.as_str());
                    println!("Entire server response is: {:?}", s);
                },
                ResponseType::Error(e) => {
                    // The api returned some sort of error:
                    println!("The API response: {}", e.error);
                }
            }
        }
    }
}

// Example function to show available credits
async fn credits(api: &Api) {

    // get remaining credits:
    let response = api.get_credits().await;

    match response {
        Err(error) => {
            // this means we got an error during the http call or after
            println!("Error: {}", error);
        },
        Ok(response) => {
            // this means our http call was ok
            match response {
                ResponseType::Success(s) => {
                    // the API call ran okay, we have a response
                    println!("You have {} credits left", s.get_credits());
                },
                ResponseType::Error(e) => {
                    // The api returned some sort of error:
                    println!("The API response: {}", e.error);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // instantiate the api
    let api = Api::new("your-api-key");

    // output the result of validation call for a valid email address
    validate(&api, "valid@example.com").await;

    // output the result of validation call for an invalid email address
    validate(&api, "invalid@example.com").await;

    // output the result of the get credits call
    credits(&api).await;

    Ok(())
}
```

## Testing  
Set your api key in the `ZEROBOUNCE_API_KEY` environment variable, then run `cargo test`.
