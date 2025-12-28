use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ResponseData<T> {
    pub status: bool,
    pub msg: String,
    pub data: T,
}

#[derive(Serialize)]
pub struct ResponseEmpty {
    pub status: bool,
    pub msg: String,
}
