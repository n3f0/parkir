use actix_web::{HttpResponse, get};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("<h1>Sistim Pengelolaan Parkir Digital</h1>")
}

#[get("/about")]
pub async fn about() -> HttpResponse {
    HttpResponse::Ok().body("<h1>About</h1>")
}
