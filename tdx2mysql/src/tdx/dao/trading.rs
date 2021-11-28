//use sqlx::{Error, MySql, Pool, FromRow};
//use sqlx::FromRow;
//use sqlx::mysql::MySqlQueryResult;
//use serde::{Deserialize, Serialize};
//use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime};

//#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
#[derive(Debug, Clone)]
pub struct TradingDataType {
    pub type_code: String,
    pub stock_code: String,
    pub title: String,
    pub trading_cycle: String,
    pub restoration: String,
    pub precision: i32,
}

//#[derive(Debug, FromRow, Clone)]
#[derive(Debug, Clone)]
pub struct TradingData {
    pub id: u64,
    pub trade_code: String,
    pub trade_date: chrono::NaiveDateTime,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume: u64,
    pub amount:f64,
}

impl TradingData {
    pub fn save_db(self, conn : &mut sqlx::mysql::MySqlConnection) {
        let sql = r#"insert into trade_data (trade_code, trade_date, open, high, low, close, volume, amount) value (?, ?, ?, ?, ?, ?, ?, ?);"#;
        let affect_rows = sqlx::query(sql)
                            .bind(self.trade_code).bind(self.trade_date)
                            .bind(self.open).bind(self.high).bind(self.low).bind(self.close)
                            .bind(self.volume).bind(self.amount).execute(conn);
    }
}