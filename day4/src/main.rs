use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = "data/input";
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
}
