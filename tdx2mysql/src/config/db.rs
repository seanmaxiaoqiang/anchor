use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

pub async fn do_connect() -> Pool<MySql> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://sean:Dir123dir@centos8:3306/tdx").await;
       // .connect("mysql://sulin:databenD!9@localhost:3306/shorten_db").await;
    pool.unwrap()
}
