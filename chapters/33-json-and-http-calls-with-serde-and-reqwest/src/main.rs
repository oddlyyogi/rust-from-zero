use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Parcel {
    id: u32,
    town: String,
    weight_grams: u32,
}

#[derive(Deserialize)]
struct Echo {
    json: Parcel,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let parcel = Parcel {
        id: 7,
        town: String::from("Oslo"),
        weight_grams: 1200,
    };
    let echo: Echo = client
        .post("https://postman-echo.com/post")
        .json(&parcel)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    println!("{:?}", echo.json);
    Ok(())
}
