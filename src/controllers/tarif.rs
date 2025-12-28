use crate::{controllers::response::ResponseData, models::tarif::Tarif};
use actix_web::{HttpResponse, get, web};
use deadpool_postgres::Pool;

#[get("/tarif/{tahun}")]
pub async fn index(pool: web::Data<Pool>, tahun: web::Path<i32>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let rows = client
        .query(
            "select id, tahun, awal, kelipatan, jenis from m_tarif where tahun=$1 order by id",
            &[&*tahun],
        )
        .await
        .expect("Gagal mengeksekusi query");

    let tarifs: Vec<Tarif> = rows.into_iter().map(Tarif::from).collect();
    let response = ResponseData {
        status: true,
        msg: "Sukses".to_string(),
        data: tarifs,
    };
    HttpResponse::Ok().json(response)
}
