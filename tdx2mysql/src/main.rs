extern crate encoding;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use encoding::all::GBK;
use encoding::{Encoding, EncoderTrap, DecoderTrap};

fn main() {
     let file = File::open("./tdx/SH#999999.txt").expect("file not found");
    let reader = BufReader::new(&file);

    let lines = reader.split(b'\n').map(|l| l.unwrap());

    for line in lines {
        let decoded_string = GBK.decode(&line, DecoderTrap::Strict).unwrap();
        println!("{}", decoded_string);
    }
}