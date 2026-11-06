use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[derive(Clone, Serialize)]
struct Parcel {
    id: u32,
    town: String,
}

#[derive(Deserialize)]
struct NewParcel {
    town: String,
}

type Db = Arc<Mutex<Vec<Parcel>>>;

async fn hello() -> &'static str {
    "Harbor Parcels is open"
}

async fn list(State(db): State<Db>) -> Json<Vec<Parcel>> {
    let parcels = db.lock().unwrap();
    Json(parcels.clone())
}

async fn create(State(db): State<Db>, Json(new): Json<NewParcel>) -> (StatusCode, Json<Parcel>) {
    let mut parcels = db.lock().unwrap();
    let parcel = Parcel {
        id: parcels.len() as u32 + 1,
        town: new.town,
    };
    parcels.push(parcel.clone());
    (StatusCode::CREATED, Json(parcel))
}

async fn find(State(db): State<Db>, Path(id): Path<u32>) -> Result<Json<Parcel>, StatusCode> {
    let parcels = db.lock().unwrap();
    parcels
        .iter()
        .find(|p| p.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(Vec::new()));
    let app = Router::new()
        .route("/", get(hello))
        .route("/parcels/{id}", get(find))
        .route("/parcels", get(list).post(create))
        .with_state(db);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
