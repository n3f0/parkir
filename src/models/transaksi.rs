use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Transaksi {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub id_user: i32,
    pub id_jukir: i32,
    pub amount: f64,
    pub id_parkir: i32,
}

impl From<Row> for Transaksi {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            start: row.get("start"),
            end: row.get("end"),
            id_user: row.get("id_user"),
            id_jukir: row.get("id_jukir"),
            amount: row.get("amount"),
            id_parkir: row.get("id_parkir"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateTransaksi {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub id_user: i32,
    pub id_jukir: i32,
    pub amount: f64,
    pub id_parkir: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTransaksi {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub id_user: i32,
    pub id_jukir: i32,
    pub amount: f64,
    pub id_parkir: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteTransaksi {
    pub id: i32,
}