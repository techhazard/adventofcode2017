extern crate regex;
use regex::Regex;
use std;

use instruction::{Condition, Operator, Command, Instruction};

/// use this macro to assign matches to your variables
/// using destructuring let's
///
/// N.B.: the match must exist, othewise it will panic!
/// TODO: no longer panic!
///
/// # Examples
///
/// ```
/// let (address,) = matches!(capture_groups, 1);
/// ```
///
/// ```
/// let (matched_address, matched_operator, matched_value) = matches!(capture_groups, 4, 5, 6);
/// ```
macro_rules! matches {
    ( $name:tt, $( $x:expr ),* ) => {
        {
            ($($name.get($x).unwrap().as_str(),)*)
        }
    };
}


pub fn parse_instruction(instruction_line : std::result::Result<std::string::String, std::io::Error> ) -> Instruction {

	let instruction_line = match instruction_line {
		Ok(string) => string,
		Err(_) => panic!("aaaah"),
	};

    let re = Regex::new(r"^(\w+) (inc|dec) (-?\d+) if (\w+) ([><=!]+) (-?\d+)$").unwrap();

    let instruction = if let Some(capture_groups) = re.captures(&instruction_line) {

        let (matched_address,) = matches!(capture_groups, 1);
        let address = matched_address.to_owned();

        let (matched_command, matched_value) =  matches!(capture_groups, 2, 3);
        let value : i64 = matched_value.parse().unwrap();

        let command = match matched_command {
            "inc" => Command::Increment(value),
            "dec" => Command::Decrement(value),
            _ => unreachable!(),
        };


        let condition_matches = matches!(capture_groups, 4, 5, 6);
        let condition = parse_condition(condition_matches);

        Instruction { address: address.to_owned(), command: command, condition: condition }

    } else { panic!() };

    instruction
}

fn parse_condition(condition_matches : (&str, &str, &str)) -> Condition {
    let (matched_address, matched_operator, matched_value) = condition_matches;

    let address = matched_address.to_owned();

    let operator : Operator =
        match matched_operator {
        ">" =>  Operator::GreaterThan,
        "<" =>  Operator::LessThan,
        ">=" => Operator::GreaterThanOrEqual,
        "<=" => Operator::LessThanOrEqual,
        "==" => Operator::Equal,
        "!=" => Operator::NotEqual,
        _ => wtf!()
    };

    let value : i64 = matched_value.parse().unwrap();

    Condition { operator, address, value}
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Command::*;
    use super::Operator::*;
	use std::io::Result;
	use std::io::Error;

    #[test]
    fn test_parse_instruction() {
        let instruction_line = "b inc 5 if a > 1".to_owned();
        let instruction = parse_instruction(Ok(instruction_line));

        let exp_cond = Condition{address: "a".to_owned(), operator: GreaterThan, value: 1};
        let expected_instruction = Instruction{address: "b".to_owned(), command: Increment(5), condition: exp_cond};

        assert_eq!(instruction, expected_instruction);
    }
    #[test]
    fn test_parse_instruction2() {
        let instruction_line = "c dec -10 if a >= 1".to_owned();
        let instruction = parse_instruction(Ok(instruction_line));

        let exp_cond = Condition{address: "a".to_owned(), operator: GreaterThanOrEqual, value: 1};
        let expected_instruction = Instruction{address: "c".to_owned(), command: Decrement(-10), condition: exp_cond};

        assert_eq!(instruction, expected_instruction);
    }
    #[test]
	#[should_panic]
    fn test_parse_instruction3() {
        let instruction_line = "b inc 5 if a > 1".to_owned();
        let instruction = parse_instruction(Err(Error::new(ErrorKind::Other, "oh no!")));
    }
}
