extern crate encoding;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use encoding::all::GBK;
use encoding::{Encoding, EncoderTrap, DecoderTrap};

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
    match line_num {
        1 => data_header(line),
        2 => data_title(line),
        _ => one_data(line_num, line.trim_matches('\r')),
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

fn one_data(line_num:u32, line:&str) {
    let ts = line.split(",");
    print!("{}::\"{}\"::[{}:",line_num-2, line, ts.clone().count());
    for s in ts {
        print!("{}:", s)
    }
    println!("]")
}