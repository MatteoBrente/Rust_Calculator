use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse::<f32>().unwrap();

    println!("{:?}", output(first_number, operator, second_number, operate(operator, first_number, second_number)));
}

fn operate(operator: char, first: f32, second: f32) -> f32 {
    if operator == '+' {
        return first + second;
    }
    else if operator == '-' {
        return first - second;
    }
    else if operator == 'x' || operator == 'X' || operator == '*' {
        return first * second;
    }
    else if operator == ':' || operator == '/' {
        return  first / second;
    }
    else {
        panic!("Invalid operator")
    }
}

fn output(first: f32, operator: char, second: f32, result: f32) -> String {
    return format!("{} {} {} = {}", first, operator, second, result);
}