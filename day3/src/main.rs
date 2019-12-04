use std::fs::File;
use std::io::{BufReader, BufRead, Read};

fn main() {
    let mut wire1: Vec<String> = Vec::new();
    let mut wire2: Vec<String> = Vec::new();

    let mut f1 = File::open("wire1.txt").unwrap();
    let mut f2 = File::open("wire2.txt").unwrap();

    let mut wire_1_inputs = String::new();
    let mut wire_2_inputs = String::new();
    f1.read_to_string(&mut wire_1_inputs);
    f2.read_to_string(&mut wire_2_inputs);

    for s in wire_1_inputs.split(",") {
        wire1.push(s.to_string());
    }

    for s in wire_2_inputs.split(",") {
        wire2.push(s.to_string());
    }
    println!("wire 1 : {:?}", wire1);
    println!("wire 2 : {:?}", wire2);
}
