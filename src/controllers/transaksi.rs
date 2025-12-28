use actix_web::{HttpResponse, get, post, web};
use deadpool_postgres::Pool;

use crate::{
    controllers::response::{ResponseData, ResponseEmpty},
    models::transaksi::{CreateTransaksi, DeleteTransaksi, Transaksi, UpdateTransaksi},
};

#[get("/transaksi")]
pub async fn index(pool: web::Data<Pool>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let rows = client
        .query(
            "select id, start::timestamp, \"end\"::timestamp, id_user, id_jukir, amount, id_parkir from d_transaksi order by id desc",
            &[],
        )
        .await
        .expect("Gagal mengeksekusi query");

    let transaksi: Vec<Transaksi> = rows.into_iter().map(Transaksi::from).collect();
    let response = ResponseData {
        status: true,
        msg: "Sukses".to_string(),
        data: transaksi,
    };
    HttpResponse::Ok().json(response)
}

#[post("/transaksi/buat")]
pub async fn create(pool: web::Data<Pool>, payload: web::Json<CreateTransaksi>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let statement = client
        .prepare("insert into d_transaksi (start, \"end\", id_user, id_jukir, amount, id_parkir) values ($1, $2, $3, $4, $5, $6)")
        .await
        .expect("Gagal mempersiapkan statement");

    let _ = client
        .execute(
            &statement,
            &[
                &payload.start,
                &payload.end,
                &payload.id_user,
                &payload.id_jukir,
                &payload.amount,
                &payload.id_parkir,
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

#[post("/transaksi/ubah")]
pub async fn change(pool: web::Data<Pool>, payload: web::Json<UpdateTransaksi>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let statement = client
        .prepare("update d_transaksi set start=$1, \"end\"=$2, id_user=$3, id_jukir=$4, amount=$5, id_parkir=$6 where id=$7")
        .await
        .expect("Gagal mempersiapkan statement");

    let _ = client
        .execute(
            &statement,
            &[
                &payload.start,
                &payload.end,
                &payload.id_user,
                &payload.id_jukir,
                &payload.amount,
                &payload.id_parkir,
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

#[post("/transaksi/hapus")]
pub async fn remove(pool: web::Data<Pool>, payload: web::Json<DeleteTransaksi>) -> HttpResponse {
    let client = pool.get().await.expect("Gagal terhubung ke database");

    let statement = client
        .prepare("delete from d_transaksi where id=$1")
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
