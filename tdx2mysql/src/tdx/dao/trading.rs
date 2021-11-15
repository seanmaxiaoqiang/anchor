use sqlx::{Error, MySql, Pool, FromRow};
use sqlx::mysql::MySqlQueryResult;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct data_type {
    pub type_code: String,
    pub stock_code: String,
    pub title: String,
    pub trading_cycle: String,
    pub restoration: String,
    pub precision: i32,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct data {
    pub id: u64,
    pub trade_code: String,
    pub trade_date: Datetime,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume: u64,
    pub amount:f64,
}
