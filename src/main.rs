// use std::io::{self, Write};
// use std::process;

// #[derive(Debug)]
// enum CalculatorError {
//     InvalidInput(String),
//     DivisionByZero,
//     UnknownOperation(String),
// }

// #[derive(Debug, Clone, Copy)]
// enum Operation {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
//     Modulo,
// }

// impl Operation {
//     fn from_symbol(symbol: &str) -> Result<Self, CalculatorError> {
//         match symbol {
//             "+" | "add" => Ok(Operation::Add),
//             "-" | "sub" => Ok(Operation::Subtract),
//             "*" | "x" | "mul" => Ok(Operation::Multiply),
//             "/" | "div" => Ok(Operation::Divide),
//             "%" | "mod" => Ok(Operation::Modulo),
//             _ => Err(CalculatorError::UnknownOperation(symbol.to_string())),
//         }
//     }

//     fn apply(self, a: f64, b: f64) -> Result<f64, CalculatorError> {
//         match self {
//             Operation::Add => Ok(a + b),
//             Operation::Subtract => Ok(a - b),
//             Operation::Multiply => Ok(a * b),
//             Operation::Divide => {
//                 if b == 0.0 {
//                     Err(CalculatorError::DivisionByZero)
//                 } else {
//                     Ok(a / b)
//                 }
//             }
//             Operation::Modulo => {
//                 if b == 0.0 {
//                     Err(CalculatorError::DivisionByZero)
//                 } else {
//                     Ok(a % b)
//                 }
//             }
//         }
//     }
// }

// struct Calculator;

// impl Calculator {
//     fn new() -> Self {
//         Calculator
//     }

//     fn parse_number(input: &str) -> Result<f64, CalculatorError> {
//         input
//             .trim()
//             .parse()
//             .map_err(|_| CalculatorError::InvalidInput(input.to_string()))
//     }

//     fn print_help() {
//         println!("\n╔══════════════════════════════════════════╗");
//         println!("║         RUST CLI CALCULATOR v1.0         ║");
//         println!("╠══════════════════════════════════════════╣");
//         println!("║  Supported Operations:                    ║");
//         println!("║    +  or add   : Addition                 ║");
//         println!("║    -  or sub   : Subtraction              ║");
//         println!("║    *  or x     : Multiplication            ║");
//         println!("║    /  or div   : Division                 ║");
//         println!("║    %  or mod   : Modulo                   ║");
//         println!("╠══════════════════════════════════════════╣");
//         println!("║  Commands:                                ║");
//         println!("║    help         : Show this menu          ║");
//         println!("║    clear        : Clear screen            ║");
//         println!("║    exit         : Exit calculator         ║");
//         println!("╚══════════════════════════════════════════╝");
//         println!("  Format: <number1> <operation> <number2>");
//         println!("  Example: 10 + 5");
//         println!("  Example: 3.5 * 2");
//         println!();
//     }

//     fn print_error(error: &CalculatorError) {
//         let error_msg = match error {
//             CalculatorError::InvalidInput(input) => format!("Invalid number: '{}'", input),
//             CalculatorError::DivisionByZero => "Error: Division by zero is not allowed".to_string(),
//             CalculatorError::UnknownOperation(op) => format!("Unknown operation: '{}'", op),
//         };
//         eprintln!("\n❌ {}", error_msg);
//     }

//     fn clear_screen() {
//         print!("{}[2J{}[1;1H", 27 as char, 27 as char);
//         io::stdout().flush().unwrap();
//     }

//     fn calculate(&self, input: &str) -> Result<f64, CalculatorError> {
//         let parts: Vec<&str> = input.trim().split_whitespace().collect();

//         if parts.len() != 3 {
//             return Err(CalculatorError::InvalidInput(input.to_string()));
//         }

//         let a = Self::parse_number(parts[0])?;
//         let op = Operation::from_symbol(parts[1])?;
//         let b = Self::parse_number(parts[2])?;

//         op.apply(a, b)
//     }

//     fn format_result(&self, result: f64) -> String {
//         if result.fract() == 0.0 {
//             format!("{}", result as i64)
//         } else {
//             format!("{:.6}", result).trim_end_matches('0').to_string()
//         }
//     }

//     fn run(&self) {
//         Self::print_help();

//         loop {
//             print!("Enter expression (or 'help'): ");
//             io::stdout().flush().unwrap();

//             let mut input = String::new();
//             if io::stdin().read_line(&mut input).is_err() {
//                 eprintln!("Error reading input");
//                 continue;
//             }

//             let input = input.trim();
//             if input.is_empty() {
//                 continue;
//             }

//             match input.to_lowercase().as_str() {
//                 "exit" | "quit" | "q" => {
//                     println!("\n👋 Goodbye!");
//                     process::exit(0);
//                 }
//                 "help" | "h" | "?" => {
//                     Self::print_help();
//                     continue;
//                 }
//                 "clear" | "cls" => {
//                     Self::clear_screen();
//                     continue;
//                 }
//                 _ => {
//                     match self.calculate(input) {
//                         Ok(result) => {
//                             println!("\n✓ Result: {}", self.format_result(result));
//                         }
//                         Err(error) => {
//                             Self::print_error(&error);
//                         }
//                     }
//                 }
//             }
//             println!();
//         }
//     }
// }

// fn main() {
//     let calculator = Calculator::new();
//     calculator.run();
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_number() {
//         assert_eq!(Calculator::parse_number("5").unwrap(), 5.0);
//         assert_eq!(Calculator::parse_number("3.14").unwrap(), 3.14);
//         assert_eq!(Calculator::parse_number("-10").unwrap(), -10.0);
//         assert!(Calculator::parse_number("abc").is_err());
//     }

//     #[test]
//     fn test_operation_add() {
//         assert_eq!(Operation::Add.apply(5.0, 3.0).unwrap(), 8.0);
//         assert_eq!(Operation::Add.apply(-2.0, 10.0).unwrap(), 8.0);
//     }

//     #[test]
//     fn test_operation_subtract() {
//         assert_eq!(Operation::Subtract.apply(10.0, 4.0).unwrap(), 6.0);
//         assert_eq!(Operation::Subtract.apply(5.0, 10.0).unwrap(), -5.0);
//     }

//     #[test]
//     fn test_operation_multiply() {
//         assert_eq!(Operation::Multiply.apply(4.0, 3.0).unwrap(), 12.0);
//         assert_eq!(Operation::Multiply.apply(-2.0, 5.0).unwrap(), -10.0);
//     }

//     #[test]
//     fn test_operation_divide() {
//         assert_eq!(Operation::Divide.apply(10.0, 2.0).unwrap(), 5.0);
//         assert!(Operation::Divide.apply(10.0, 0.0).is_err());
//     }

//     #[test]
//     fn test_operation_modulo() {
//         assert_eq!(Operation::Modulo.apply(10.0, 3.0).unwrap(), 1.0);
//         assert!(Operation::Modulo.apply(10.0, 0.0).is_err());
//     }

//     #[test]
//     fn test_operation_from_symbol() {
//         assert!(matches!(Operation::from_symbol("+"), Ok(Operation::Add)));
//         assert!(matches!(Operation::from_symbol("add"), Ok(Operation::Add)));
//         assert!(matches!(Operation::from_symbol("-"), Ok(Operation::Subtract)));
//         assert!(matches!(Operation::from_symbol("*"), Ok(Operation::Multiply)));
//         assert!(matches!(Operation::from_symbol("x"), Ok(Operation::Multiply)));
//         assert!(matches!(Operation::from_symbol("/"), Ok(Operation::Divide)));
//         assert!(matches!(Operation::from_symbol("%"), Ok(Operation::Modulo)));
//         assert!(Operation::from_symbol("invalid").is_err());
//     }
// }


// fn main() {
// 	// let a = String::from("Hello");
// 	// let b = a.clone();
// 	// let c = b.clone();
//     // println!("a: {}, b: {}, c: {}", a, b, c);

//     let s1 = String::from("Hello");
//     let s2 = s1.clone(); // clone s1 before moving it to s2
//     println!("s1: {}", s1);
// }

// fn print_length(s: &String) {
//     println!("Length = {}",s.len());
// }

// fn append_world(s: &mut String) {
//     s.push_str(" World");
// }

// fn main() {
//     let mut greeting = String::from("Hello");
//     print_length(&greeting);
//     append_world(&mut greeting);
//     println!("Updated string: {}", greeting);
// }

mod food_registry;

use food_registry::{FoodRegistry, FoodDetails, FoodType};

fn main() {
    let mut registry = FoodRegistry::new();

    // Add snacks
    let _ = registry.add_food(FoodDetails {
        name: "Swallow".to_string(),
        food_type: FoodType::Snack,
        calories: 150,
        price: 2.50,
    });

    let _ = registry.add_food(FoodDetails {
        name: "Beans".to_string(),
        food_type: FoodType::Snack,
        calories: 200,
        price: 3.00,
    });

    // Add drinks
    let _ = registry.add_food(FoodDetails {
        name: "Water".to_string(),
        food_type: FoodType::Drink,
        calories: 0,
        price: 1.00,
    });

    // Retrieve and display
    match registry.get_food("Swallow") {
        Ok(food) => println!("Found: {:?}", food),
        Err(e) => println!("Error: {:?}", e),
    }
}