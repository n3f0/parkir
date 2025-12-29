use actix_web::{HttpResponse, get, post, web};
use deadpool_postgres::Pool;
use uuid::Uuid;

use crate::{
    controllers::response::{ResponseData, ResponseEmpty},
    models::jukir::{CreateJukir, DeleteJukir, Jukir, UpdateJukir},
};

#[get("/jukir")]
pub async fn index(pool: web::Data<Pool>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let rows = client
        .query(
            "select id, nama, no_ktp, alamat, no_hp, unique_key from m_juru_parkir order by id desc",
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

#[post("/jukir/buat")]
pub async fn create(pool: web::Data<Pool>, payload: web::Json<CreateJukir>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");
    let random_string = Uuid::new_v4().to_string();

    let statement = client
        .prepare("insert into m_juru_parkir (nama, no_ktp, alamat, no_hp, unique_key) values ($1, $2, $3, $4, $5)")
        .await
        .expect("Gagal mempersiapkan statement");

    let _ = client
        .execute(
            &statement,
            &[
                &payload.nama,
                &payload.no_ktp,
                &payload.alamat,
                &payload.no_hp,
                &random_string,
            ],
        )
        .await
        .expect("Gagal mengeksekusi insert");

    let response = ResponseEmpty {
        status: true,
        msg: "Sukses Menambah Data".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("/jukir/ubah")]
pub async fn change(pool: web::Data<Pool>, payload: web::Json<UpdateJukir>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let statement = client
        .prepare("update m_juru_parkir set nama=$1, no_ktp=$2, alamat=$3, no_hp=$4 where id=$5")
        .await
        .expect("Gagal mempersiapkan statement");

    let _ = client
        .execute(
            &statement,
            &[
                &payload.nama,
                &payload.no_ktp,
                &payload.alamat,
                &payload.no_hp,
                &payload.id,
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

#[post("/jukir/hapus")]
pub async fn remove(pool: web::Data<Pool>, payload: web::Json<DeleteJukir>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let statement = client
        .prepare("delete from m_juru_parkir where id=$1")
        .await
        .expect("Gagal mempersiapkan statement");

    let _ = client
        .execute(&statement, &[&payload.id])
        .await
        .expect("Gagal mengeksekusi delete");

    let response = ResponseEmpty {
        status: true,
        msg: "Sukses Menghapus Data".to_string(),
    };
    HttpResponse::Ok().json(response)
}
