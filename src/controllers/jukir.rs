use actix_web::{HttpResponse, get, web};
use deadpool_postgres::Pool;

use crate::{controllers::response::ResponseData, models::jukir::Jukir};

#[get("/jukir")]
pub async fn index(pool: web::Data<Pool>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let rows = client
        .query(
            "select id, nama, no_ktp, alamat, no_hp, unique_key from m_juru_parkir order by id",
            &[],
        )
        .await
        .expect("Gagal mengeksekusi query");

    let jukirs: Vec<Jukir> = rows.into_iter().map(Jukir::from).collect();
    let response = ResponseData {
        status: true,
        msg: "Sukses".to_string(),
        data: jukirs,
    };
    HttpResponse::Ok().json(response)
}
