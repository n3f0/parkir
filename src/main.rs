use std::io;

use actix_web::{App, HttpServer, web};

mod controllers;
mod dbpool;
mod models;
mod route;
struct WebService {
    host: String,
    port: u16,
}

struct DbConn {
    host: String,
    port: u16,
    user: String,
    pass: String,
    dbname: String,
    schema: String,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    //konfigurasi server web
    //konfigurasi server web
    let web_config: WebService = WebService {
        host: "0.0.0.0".to_string(),
        port: 8080,
    };
    //end konfigurasi server web

    //konfigurasi database
    let conn: DbConn = DbConn {
        host: "localhost".to_string(),
        port: 5432,
        user: "postgres".to_string(),
        pass: "nokiae90".to_string(),
        dbname: "parkir".to_string(),
        schema: "public".to_string(),
    };
    //end konfigurasi database

    let pool = dbpool::new(
        conn.host,
        conn.port,
        conn.user,
        conn.pass,
        conn.dbname,
        conn.schema,
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(route::route)
    })
    .bind((web_config.host, web_config.port))?
    .run()
    .await
}
