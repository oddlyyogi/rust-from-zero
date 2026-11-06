# 33 - JSON and HTTP Calls with serde and reqwest

How do programs share data? A printed form passed between two depots explains JSON. We turn Rust structs into JSON and back with serde, see bad input stopped at the door, handle names that don't match, call a real weather service with reqwest, read the answer into our own types, check the status before reading the body, set a timeout, make three calls at once, and send JSON with a POST.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo add serde --features derive
cargo add serde_json
cargo run
cargo add reqwest --features json
cargo add tokio --features full
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
fn main() {
}
```

Compiles and runs:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Parcel {
    id: u32,
    town: String,
    weight_grams: u32,
}

fn main() {
    let parcel = Parcel {
        id: 7,
        town: String::from("Oslo"),
        weight_grams: 1200,
    };
    let text = serde_json::to_string(&parcel).unwrap();
    println!("{text}");
}
```

Compiles and runs:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Parcel {
    id: u32,
    town: String,
    weight_grams: u32,
}

fn main() {
    let parcel = Parcel {
        id: 7,
        town: String::from("Oslo"),
        weight_grams: 1200,
    };
    let text = serde_json::to_string_pretty(&parcel).unwrap();
    println!("{text}");
}
```

Compiles and runs:

```rust
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Parcel {
    id: u32,
    town: String,
    weight_grams: u32,
}

fn main() {
    let text = r#"{"id": 7, "town": "Oslo", "weight_grams": 1200, "colour": "red"}"#;
    let parcel: Parcel = serde_json::from_str(text).unwrap();
    println!("{parcel:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Parcel {
    id: u32,
    town: String,
    weight_grams: u32,
}

fn main() {
    let text = r#"{"id": 7, "town": "Oslo", "colour": "red"}"#;
    let parcel: Parcel = serde_json::from_str(text).unwrap();
    println!("{parcel:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Parcel {
    id: u32,
    town: String,
    weight_grams: u32,
}

fn main() {
    let text = r#"{"id": 7, "town": "Oslo", "weight_grams": "heavy"}"#;
    let parcel: Parcel = serde_json::from_str(text).unwrap();
    println!("{parcel:?}");
}
```

Compiles and runs:

```rust
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Parcel {
    id: u32,
    town: String,
    weight_grams: u32,
    note: Option<String>,
    #[serde(default)]
    insured: bool,
}

fn main() {
    let text = r#"{"id": 7, "town": "Oslo", "weightGrams": 1200}"#;
    let parcel: Parcel = serde_json::from_str(text).unwrap();
    println!("{parcel:?}");
}
```

Compiles and runs:

```rust
use serde_json::Value;

fn main() {
    let data: Value = serde_json::from_str(r#"{"town": "Oslo"}"#).unwrap();
    println!("{}", data["town"]);
    println!("{}", data["missing"]);
}
```

Compiles and runs:

```rust
use serde_json::{Value, json};

fn main() {
    let data: Value = serde_json::from_str(r#"{"town": "Oslo"}"#).unwrap();
    println!("{}", data["town"]);
    println!("{}", data["missing"]);
    let body = json!({"town": "Oslo", "urgent": true});
    println!("{body}");
}
```

Compiles and runs:

```rust
const URL: &str =
    "https://api.open-meteo.com/v1/forecast?latitude=59.91&longitude=10.75&current_weather=true";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(URL).await?;
    println!("{}", response.status());
    let text = response.text().await?;
    println!("{}", &text[..80]);
    Ok(())
}
```

Compiles and runs:

```rust
use serde::Deserialize;

const URL: &str =
    "https://api.open-meteo.com/v1/forecast?latitude=59.91&longitude=10.75&current_weather=true";

#[derive(Deserialize)]
struct Forecast {
    current_weather: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let forecast: Forecast = reqwest::get(URL).await?.json().await?;
    let now = forecast.current_weather;
    println!("Oslo: {} C, wind {} km/h", now.temperature, now.windspeed);
    Ok(())
}
```

Does not compile, on purpose, so we can read the error:

```rust
use serde::Deserialize;

const URL: &str =
    "https://api.open-meteo.com/v1/forecast?latitude=999&longitude=10.75&current_weather=true";

#[derive(Deserialize)]
struct Forecast {
    current_weather: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let forecast: Forecast = reqwest::get(URL).await?.json().await?;
    let now = forecast.current_weather;
    println!("Oslo: {} C, wind {} km/h", now.temperature, now.windspeed);
    Ok(())
}
```

Does not compile, on purpose, so we can read the error:

```rust
use serde::Deserialize;

const URL: &str =
    "https://api.open-meteo.com/v1/forecast?latitude=999&longitude=10.75&current_weather=true";

#[derive(Deserialize)]
struct Forecast {
    current_weather: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let forecast: Forecast = reqwest::get(URL)
        .await?
        .error_for_status()?
        .json()
        .await?;
    let now = forecast.current_weather;
    println!("Oslo: {} C, wind {} km/h", now.temperature, now.windspeed);
    Ok(())
}
```

Compiles and runs:

```rust
use serde::Deserialize;

const URL: &str =
    "https://api.open-meteo.com/v1/forecast?latitude=59.91&longitude=10.75&current_weather=true";

#[derive(Deserialize)]
struct Forecast {
    current_weather: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let forecast: Forecast = reqwest::get(URL)
        .await?
        .error_for_status()?
        .json()
        .await?;
    let now = forecast.current_weather;
    println!("Oslo: {} C, wind {} km/h", now.temperature, now.windspeed);
    Ok(())
}
```

Compiles and runs:

```rust
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
struct Forecast {
    current_weather: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature: f64,
}

async fn weather(
    client: &Client,
    town: &str,
    lat: f64,
    lon: f64,
) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}\
         &current_weather=true"
    );
    let forecast: Forecast = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(format!("{town}: {} C", forecast.current_weather.temperature))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let (a, b, c) = tokio::join!(
        weather(&client, "Oslo", 59.91, 10.75),
        weather(&client, "Bergen", 60.39, 5.32),
        weather(&client, "Tromso", 69.65, 18.96),
    );
    println!("{}", a?);
    println!("{}", b?);
    println!("{}", c?);
    Ok(())
}
```

Compiles and runs:

```rust
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
```

## Sources

- serde derive and attributes: https://serde.rs/derive.html and https://serde.rs/attributes.html
- serde ignores unknown fields by default, deny_unknown_fields opts out: https://serde.rs/container-attrs.html
- serde_json Value and json macro: https://docs.rs/serde_json/latest/serde_json/
- reqwest::get is for one-off requests, Client pools connections: https://docs.rs/reqwest/latest/reqwest/
- error_for_status: https://docs.rs/reqwest/latest/reqwest/struct.Response.html#method.error_for_status
- Open-Meteo forecast API, no key for non-commercial use: https://open-meteo.com/en/docs
- Postman Echo: https://www.postman.com/postman/published-postman-templates/documentation/ae2ja6x/postman-echo
