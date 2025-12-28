use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Parkir {
    pub id_parkir: i32,
    pub lokasi: String,
    pub lat: f32,
    pub lon: f32,
    pub unique_key: String,
}

impl From<Row> for Parkir {
    fn from(row: Row) -> Self {
        Self {
            id_parkir: row.get("id_parkir"),
            lokasi: row.get("lokasi"),
            lat: row.get("lat"),
            lon: row.get("lon"),
            unique_key: row.get("unique_key"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateParkir {
    pub lokasi: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateParkir {
    pub id_parkir: i32,
    pub lokasi: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteParkir {
    pub id_parkir: i32,
}
