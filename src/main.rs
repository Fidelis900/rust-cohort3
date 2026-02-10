mod calculator;

use std::io::{self, Write};

fn main() {
    println!("Welcome to the Calculator CLI!");
    println!("Available operations: add, subtract, multiply, divide, sqrt");
    println!("Format: <operation> <num1> [num2]");
    println!("Example: add 5 3");
    println!("Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 2 {
            println!("Invalid input. Please enter operation and numbers.");
            continue;
        }

        let op = parts[0];
        match op {
            "add" | "subtract" | "multiply" | "divide" => {
                if parts.len() != 3 {
                    println!("Invalid input. Need two numbers for {}", op);
                    continue;
                }
                let a: f64 = match parts[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number: {}", parts[1]);
                        continue;
                    }
                };
                let b: f64 = match parts[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number: {}", parts[2]);
                        continue;
                    }
                };
                let result = match op {
                    "add" => calculator::add(a, b),
                    "subtract" => calculator::subtract(a, b),
                    "multiply" => calculator::multiply(a, b),
                    "divide" => match calculator::divide(a, b) {
                        Ok(res) => res,
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    },
                    _ => unreachable!(),
                };
                println!("Result: {}", result);
            }
            "sqrt" => {
                if parts.len() != 2 {
                    println!("Invalid input. Need one number for sqrt");
                    continue;
                }
                let a: f64 = match parts[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number: {}", parts[1]);
                        continue;
                    }
                };
                match calculator::square_root(a) {
                    Ok(res) => println!("Result: {}", res),
                    Err(e) => println!("{}", e),
                }
            }
            _ => println!("Unknown operation: {}", op),
        }
    }
}

fn user_name(name: String) {
    println!("My user name is {}", name)
}

fn add(a: u32, b: u32) -> u32 {
    let sum = a + b;

    println!("The sum of {a} and {b} is {sum}");
    return sum;
}

fn sub(a: u32, b: u32) -> u32 {
    let sum = a - b;
    add(a, b);

    println!("The sum of {a} and {b} is {sum}");
    return sum;
}

fn user(name: &str, age: u32, email: String, is_active: bool) -> String {
    println!("My user name is {}, \n age is {}, \n email is {}, \n is_active is {}", name, age, email, is_active);
    return name.to_string();
}

fn conditionals() {
    let age = 20;

    if age > 18 {
        println!("You are an adult");
    } else if age == 18 {
        println!("You just became an adult")
    } else {
        println!("you are a minor")
    }
}

fn school_conditionals() {

    let time: u32 = 19;

    if time < 8 {
        println!("You're early!")
    } else if time > 8 && time < 10 {
        println!("You're late and should be punished!")
    } else if time == 10 {
        println!("It's break time!")
    } else if time == 11 {
        println!("Break Over, Go back to class!")
    } else if time > 11 && time < 15 {
        println!("You should be in class!")
    } else if time == 15 {
        println!("It's Closing time!")
    } else {
        println!("You can do what ever you want after closing!")
    }
}

fn loops() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
        println!("Infinite loop {}", count);
    };

    println!("The result is {:?}", result);
}

fn while_loop() {
    let mut count = 6;

    while count != 0 {
        println!("The count is {}", count);
        count -= 1;
    }
}