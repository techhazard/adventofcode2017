use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn get_input_lines(filename: &str) -> Lines<BufReader<File>> {

    let file = File::open(filename).expect("not found input.txt");
    let buffer = BufReader::new(file);
    buffer.lines()

}
