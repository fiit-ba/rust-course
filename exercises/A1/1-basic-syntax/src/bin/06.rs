// Create a basic calculator program that performs mathematical operations on two integer values. The operation to be performed will be determined by a character input by the user.
//
// Requirements:
// Variable Declaration:
//
// Declare two integer variables (e.g., num1, num2) to hold the numbers for the calculation.
// Declare one character variable (e.g., operation) to specify the mathematical operation.
// User Input:
//
// Prompt the user to input two integers and store them in num1 and num2.
// Prompt the user to input a character that represents the desired operation. For instance:
// '+' for addition
// '-' for subtraction
// '*' for multiplication
// '/' for division
// Performing the Operation:
//
// Based on the character input, perform the corresponding mathematical operation on the two integers.
// Handle basic error scenarios, such as division by zero.
// Output:
//
// Display the result of the operation to the user.

use std::io;

fn main() {
    println!("enter the first integer:");
    let num1 = prompt_integer();

    println!("enter the second integer:");
    let num2 = prompt_integer();

    println!("enter an operation (+, -, *, /):");
    let operation = prompt_operation();

    match calculate(num1, num2, operation) {
        Ok(result) => {
            println!("result: {} {} {} = {}", num1, operation, num2, result);
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }
}

fn prompt_integer() -> i32 {
    // loop until user enters valid integer
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("please enter a valid integer:");
            }
        }
    }
}

fn prompt_operation() -> char {
    // loop until user enters valid operation
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let trimmed = input.trim();

        if trimmed.len() == 1 {
            let operation = trimmed.chars().next().unwrap();
            if matches!(operation, '+' | '-' | '*' | '/') {
                return operation;
            }
        }

        println!("please enter a valid operation (+, -, *, /):");
    }
}

fn calculate(num1: i32, num2: i32, operation: char) -> Result<i32, String> {
    match operation {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0 {
                Err("division by zero is not allowed".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("unsupported operation: {}", operation)),
    }
}
