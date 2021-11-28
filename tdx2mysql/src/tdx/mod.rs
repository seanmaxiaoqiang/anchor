extern crate encoding;

use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use encoding::all::GBK;
use encoding::{Encoding, DecoderTrap};

mod dao;
use dao::trading::{TradingData};

pub fn open_export_file(pathname: &str) {
    let file = File::open(pathname).expect("file not found");
    let reader = BufReader::new(&file);

    let lines = reader.split(b'\n').map(|l| l.unwrap());

    let mut line_num = 1;

    for line in lines {
        let decoded_string = GBK.decode(&line, DecoderTrap::Strict).unwrap();
        line_parser(line_num, &decoded_string);
        line_num = line_num + 1;
    }
}

fn line_parser(line_num:u32, line:&str){
    let trade_code = String::from("default");
    match line_num {
        1 => data_header(line),
        2 => data_title(line),
        _ => {
            let _r = one_data(line_num, &trade_code, line.trim_matches('\r'));
        },
    }
}

fn data_header(line:&str) {
    let ts =line.split_whitespace();
    for s in ts {
        println!("{}", s)
    }
    println!()
}

fn data_title(line:&str) {
    let ts =line.split_whitespace();
    for s in ts {
        println!("{}", s)
    }
    println!()
}

fn one_data(line_num:u32, trade_code:&str, line:&str) -> Result<TradingData, &'static str> {
    let mut ts = line.split(",");
    let fields_num = ts.clone().count();
    print!("{}::\"{}\"::[{}:",line_num-2, line, fields_num);
    if fields_num != 7 {
        return Err("Not trading data.")
    }


    let trade_date = format!("{}093000.000", ts.nth(0).unwrap());
    let trade_datetime = NaiveDateTime::parse_from_str(&trade_date, "%Y%m%d%H%M%S%.f").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&trade_datetime).unwrap();
    //let trade_date = ts.nth(0).unwrap();
    let open = ts.nth(0).unwrap().parse::<f32>().unwrap();
    let high = ts.nth(0).unwrap().parse::<f32>().unwrap();
    let low = ts.nth(0).unwrap().parse::<f32>().unwrap();
    let close = ts.nth(0).unwrap().parse::<f32>().unwrap();
    let volume = ts.nth(0).unwrap().parse::<f32>().unwrap();
    let amount = ts.nth(0).unwrap().parse::<f64>().unwrap();
    print!("{{{}, {}, {}, {}, {}, {}, {}}}", date_time, open, high, low, close, volume, amount);

    println!("]");

    let trade_data = TradingData {
        id: 0,
        trade_code: String::from(trade_code),
        trade_date: trade_datetime,
        open: (open * 100.0) as u64,
        high: (high * 100.0) as u64,
        low: (low * 100.0) as u64,
        close: (close * 100.0) as u64,
        volume: volume as u64,
        amount: amount,
    };

    Ok(trade_data)
}