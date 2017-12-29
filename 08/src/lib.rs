macro_rules! wtf {
    () => (
        panic!("What the actual F**k!");
          )
}

extern crate regex;

mod parse;
use parse::parse_instruction;
pub mod instruction;
use instruction::Instructions;
pub mod registers;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input() -> std::io::Lines<std::io::BufReader<std::fs::File>> {

    let file = File::open("input.txt").expect("not found input.txt");
    let buffer = BufReader::new(file);
    buffer.lines()

}


pub fn get_instructions() -> Instructions {

    let input_lines = get_input();

    input_lines.map(|x| parse_instruction(x) ).collect::<Instructions>()
}

