use actix_web::{HttpResponse, get, web};
use deadpool_postgres::Pool;

use crate::{controllers::response::ResponseData, models::parkir::Parkir};

#[get("/parkir")]
pub async fn index(pool: web::Data<Pool>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let rows = client
        .query(
            "select id_parkir, lokasi, lat, lon, unique_key from m_parkir order by id_parkir",
            &[],
        )
        .await
        .expect("Gagal mengeksekusi query");

    let parkirs: Vec<Parkir> = rows.into_iter().map(Parkir::from).collect();
    let response = ResponseData {
        status: true,
        msg: "Sukses".to_string(),
        data: parkirs,
    };
    HttpResponse::Ok().json(response)
}
