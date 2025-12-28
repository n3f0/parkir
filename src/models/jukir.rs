use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Jukir {
    pub id: i32,
    pub nama: String,
    pub no_ktp: String,
    pub alamat: String,
    pub no_hp: String,
    pub unique_key: String,
}

impl From<Row> for Jukir {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            nama: row.get("nama"),
            no_ktp: row.get("no_ktp"),
            alamat: row.get("alamat"),
            no_hp: row.get("no_hp"),
            unique_key: row.get("unique_key"),
        }
    }
}
