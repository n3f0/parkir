use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

pub fn new(
    host: String,
    port: u16,
    user: String,
    pass: String,
    dbname: String,
    schema: String,
) -> Pool {
    let mut config = Config::new();
    config.host = Some(host);
    config.port = Some(port);
    config.user = Some(user);
    config.password = Some(pass);
    config.dbname = Some(dbname);
    config.options = Some(format!("-c search_path={}", schema));
    config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create pool")
}
