use registers::{Address, Value};

#[derive(Debug,Eq,PartialEq)]
pub enum Command {
    Increment(Value),
    Decrement(Value)
}

#[derive(Debug,Eq,PartialEq)]
pub enum Operator {
    Equal,
    GreaterThan,
    LessThan,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual
}

#[derive(Debug,Eq,PartialEq)]
pub struct Condition {
   pub address: Address,
   pub operator: Operator,
   pub value: Value
}

#[derive(Debug,Eq,PartialEq)]
pub struct Instruction {
   pub address: Address,
   pub command: Command,
   pub condition: Condition
}

pub type Instructions = Vec<Instruction>;
