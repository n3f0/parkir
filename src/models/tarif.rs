use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Tarif {
    pub id: i32,
    pub awal: f64,
    pub kelipatan: f64,
    pub jenis: String,
}

impl From<Row> for Tarif {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            awal: row.get("awal"),
            kelipatan: row.get("kelipatan"),
            jenis: row.get("jenis"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateTarif {
    pub tahun: i32,
    pub awal: f64,
    pub kelipatan: f64,
    pub jenis: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTarif {
    pub tahun: i32,
    pub awal: f64,
    pub kelipatan: f64,
    pub jenis: String,
}
