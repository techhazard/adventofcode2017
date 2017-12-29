extern crate advent8;
use advent8::registers::{Address, Registers, Value};
use advent8::instruction::Operator::*;
use advent8::instruction::{Condition, Instruction, Command};

use std::collections::hash_map::Entry;


fn condition_holds(condition: &Condition, current_value: &Value) -> bool {
    match condition.operator {
        Equal              => *current_value == condition.value,
        GreaterThan        => *current_value >  condition.value,
        LessThan           => *current_value < condition.value,
        NotEqual           => *current_value != condition.value,
        GreaterThanOrEqual => *current_value >= condition.value,
        LessThanOrEqual    => *current_value <= condition.value,

    }
}


fn main() {
	let instructions = advent8::get_instructions();

    let mut registers = Registers::new();
    let mut max_register_value : Value = 0;

    for ref instruction in instructions.iter() {
        println!("{:?}", instruction);

        let condition_address : &Address = &instruction.condition.address;

        let condition_address_current_value : Value = registers.get(condition_address).unwrap_or(&0i64).clone();

        if condition_holds(&instruction.condition, &condition_address_current_value) {

            let address : Address = instruction.address.clone();
            let mut register_value : &mut Value = registers.entry(address).or_insert(0i64);

            match instruction.command {
                Command::Increment(val) => *register_value += val,
                Command::Decrement(val) => *register_value -= val,
            }

            max_register_value = std::cmp::max(max_register_value, *register_value);
        }

    }

    println!("{:?}", registers);
    let max_value = registers.iter().map(|(_, &x)| x).max();
    println!("{:?}", max_value);
    println!("{:?}", max_register_value);
}
