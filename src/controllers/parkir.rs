use actix_web::{HttpResponse, get, post, web};
use deadpool_postgres::Pool;
use uuid::Uuid;

use crate::{
    controllers::response::{ResponseData, ResponseEmpty},
    models::parkir::{CreateParkir, DeleteParkir, Parkir, UpdateParkir},
};

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

#[post("/parkir/buat")]
pub async fn create(pool: web::Data<Pool>, payload: web::Json<CreateParkir>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");
    let random_string = Uuid::new_v4().to_string();

    let statement = client
        .prepare("insert into m_parkir (lokasi, lat, lon, unique_key) values ($1, $2, $3, $4)")
        .await
        .expect("Gagal mempersiapkan statement");

    let _ = client
        .execute(
            &statement,
            &[&payload.lokasi, &payload.lat, &payload.lon, &random_string],
        )
        .await
        .expect("Gagal mengeksekusi insert");

    let response = ResponseEmpty {
        status: true,
        msg: "Sukses Menambah Data".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("/parkir/ubah")]
pub async fn change(pool: web::Data<Pool>, payload: web::Json<UpdateParkir>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let statement = client
        .prepare("update m_parkir set lokasi=$1, lat=$2, lon=$3 where id_parkir=$4")
        .await
        .expect("Gagal mempersiapkan statement");

    let _ = client
        .execute(
            &statement,
            &[
                &payload.lokasi,
                &payload.lat,
                &payload.lon,
                &payload.id_parkir,
            ],
        )
        .await
        .expect("Gagal mengeksekusi update");

    let response = ResponseEmpty {
        status: true,
        msg: "Sukses Merubah Data".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("/parkir/hapus")]
pub async fn remove(pool: web::Data<Pool>, payload: web::Json<DeleteParkir>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let statement = client
        .prepare("delete from m_parkir where id_parkir=$1")
        .await
        .expect("Gagal mempersiapkan statement");

    let _ = client
        .execute(&statement, &[&payload.id_parkir])
        .await
        .expect("Gagal mengeksekusi delete");

    let response = ResponseEmpty {
        status: true,
        msg: "Sukses Menghapus Data".to_string(),
    };
    HttpResponse::Ok().json(response)
}
