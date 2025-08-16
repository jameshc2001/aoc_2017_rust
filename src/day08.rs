use crate::day08::Action::{DEC, INC};

enum Action {
    INC,
    DEC,
}

impl Action {
    pub fn from_string(input: &str) -> Action {
        if input == "inc" { INC }
        else { DEC }
    }
}

struct Condition {
    register: String,
    operation: String,
    amount: i32,
}

impl Condition {
    pub fn new(register: String, operation: String, amount: i32) -> Condition {
        Condition { register, operation, amount }
    }
}

struct Instruction {
    register: String,
    action: Action,
    amount: i32,
    condition: Condition,
}

impl Instruction {
    pub fn from_string(input: String) -> Instruction {
        let parts: Vec<String> = input.split(" ").map(String::from).collect();
        let register = parts.get(0).unwrap().clone();
        let action = Action::from_string(parts.get(1).unwrap());
        let amount = parts.get(2).unwrap().parse::<i32>().unwrap();
        let condition = Condition::new(
            parts.get(4).unwrap().clone(),
            parts.get(5).unwrap().clone(),
            parts.get(6).unwrap().parse::<i32>().unwrap(),
        );
        Instruction { register, action, amount, condition }
    }
}