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

mod input;
use input::get_input_lines;

pub fn get_instructions() -> Instructions {

    let input_lines = get_input_lines("input.txt");

    input_lines.map(|x| parse_instruction(x) ).collect::<Instructions>()
}

