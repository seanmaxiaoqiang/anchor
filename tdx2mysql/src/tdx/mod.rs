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

    for line in lines {
        let decoded_string = GBK.decode(&line, DecoderTrap::Strict).unwrap();
        println!("{}", decoded_string);
    }
}
