
fn main() {
    let x:i128=22;

    //println!("Hello, world!");

   //println!("Hello, world! fib(6) = {}", fib(6));
   //variable_demo();
  //print_max_values();
  //numerical_operations_demo();
  //boolean_operations_demo();
 // comparison_operators_demo();
 //char_string_demo();
 //expression_statement_demo();
 //tuple_demo();
 //array_demo();
 //control_flow_demo();
 //let x= add_numbers_function_demo(4,4);
 //println!(" the value of x {}",x);
 scope_demo();
 //memory_owndership_demo();
}

fn fib(n: u64) -> u64 {
     if n <= 1 {
     n
     } else {
     fib(n - 1) + fib(n - 2)
    }


     
}

fn variable_demo() {
    // Immutable variable with type inference
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable with explicit type annotation
    let mut y: i32 = 10;
    println!("The value of y is: {}", y);

    // Modifying the mutable variable
    y = 15;
    println!("The modified value of y is: {}", y);

    // Shadowing (redeclaring x with a new type)
    let x = "I am a string now!";
    println!("x is now: {}", x);

    // Using a different data type
    let z: f64 = 3.14;
    println!("The value of z is: {}", z);

    // Destructuring a tuple
    let (a, b) = (10, 20);
    println!("The value of a is: {} and b is: {}", a, b);

    // Array declaration
    let arr: [i32; 3] = [1, 2, 3];
    println!("Array values: {:?}", arr);

    // Constant declaration
    const CONSTANT_VALUE: i32 = 100;
    println!("Constant value: {}", CONSTANT_VALUE);
}
fn print_max_values() {
    println!("The maximum value for i8 is {}", i8::MAX);
    println!("The maximum value for i16 is {}", i16::MAX);
    println!("The maximum value for i32 is {}", i32::MAX);
    println!("The maximum value for i64 is {}", i64::MAX);
    println!("The maximum value for i128 is {}", i128::MAX);
    println!("The maximum value for isize is {}", isize::MAX);

    println!("The maximum value for u8 is {}", u8::MAX);
    println!("The maximum value for u16 is {}", u16::MAX);
    println!("The maximum value for u32 is {}", u32::MAX);
    println!("The maximum value for u64 is {}", u64::MAX);
    println!("The maximum value for u128 is {}", u128::MAX);
    println!("The maximum value for usize is {}", usize::MAX);

    println!("The maximum value for f32 is {}", f32::MAX);
    println!("The maximum value for f64 is {}", f64::MAX);
}

fn numerical_operations_demo() {
    let a: i32 = 15;
    let b: i32 = 4;

    // Addition
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);

    // Subtraction
    let difference = a - b;
    println!("The difference between {} and {} is {}", a, b, difference);

    // Multiplication
    let product = a * b;
    println!("The product of {} and {} is {}", a, b, product);

    // Division
    let quotient = a / b;
    println!("The quotient of {} divided by {} is {}", a, b, quotient);

    // Modulus
    let remainder = a % b;
    println!("The remainder of {} divided by {} is {}", a, b, remainder);

    // Additional operations
    // Power (using i32::pow method)
    let exponent = 3;
    let power = a.pow(exponent);
    println!("{} raised to the power of {} is {}", a, exponent, power);

    // Absolute value (using i32::abs method)
    let negative_number: i32 = -10;
    let absolute = negative_number.abs();
    println!("The absolute value of {} is {}", negative_number, absolute);
}


fn boolean_operations_demo() {
    let a: bool = true;
    let b: bool = false;

    // Logical AND
    let and_result = a && b;
    println!("The result of {} AND {} is {}", a, b, and_result);

    // Logical OR
    let or_result = a || b;
    println!("The result of {} OR {} is {}", a, b, or_result);

    // Logical NOT
    let not_a = !a;
    let not_b = !b;
    println!("The result of NOT {} is {}", a, not_a);
    println!("The result of NOT {} is {}", b, not_b);

    // Combination of operations
    let combination = (a && b) || !a;
    println!("The result of ({} AND {}) OR NOT {} is {}", a, b, a, combination);
}


fn comparison_operators_demo() {
    let x: i32 = 10;
    let y: i32 = 20;

    // Equality
    let equal = x == y;
    println!("Is {} equal to {}? {}", x, y, equal);

    // Inequality
    let not_equal = x != y;
    println!("Is {} not equal to {}? {}", x, y, not_equal);

    // Greater than
    let greater_than = x > y;
    println!("Is {} greater than {}? {}", x, y, greater_than);

    // Less than
    let less_than = x < y;
    println!("Is {} less than {}? {}", x, y, less_than);

    // Greater than or equal to
    let greater_than_or_equal = x >= y;
    println!("Is {} greater than or equal to {}? {}", x, y, greater_than_or_equal);

    // Less than or equal to
    let less_than_or_equal = x <= y;
    println!("Is {} less than or equal to {}? {}", x, y, less_than_or_equal);
}

fn char_string_demo() {
    // Character declaration
    let sample_char: char = 'R';
    println!("The character is: {}", sample_char);

    // String declaration
    let mut sample_string: String = String::from("Rust");
    println!("The string is: {}", sample_string);

    // Appending a char to a String
    sample_string.push(sample_char);
    println!("After appending a char, the string is: {}", sample_string);

    // Appending a string slice
    sample_string.push_str(" Programming");
    println!("After appending a string slice, the string is: {}", sample_string);

    // Length of a String (number of bytes)
    let length = sample_string.len();
    println!("Length of the string in bytes: {}", length);

    // Accessing individual characters in a String (using chars iterator)
    println!("Individual characters in the string:");
    for c in sample_string.chars() {
        println!("{}", c);
    }

    // Slicing a String
    let slice: &str = &sample_string[0..4];
    println!("Slice of the string: {}", slice);

    // Converting String to uppercase
    let uppercase = sample_string.to_uppercase();
    println!("String in uppercase: {}", uppercase);
}
fn expression_statement_demo() {
    // Statement
    let x; // Declaration statement

    // Expression
    x = 5; // Assignment expression (Note: In Rust, assignment expressions do not return a value)

    // Expression used as a statement
    let y = { 
        let inner = 3;
        inner + 2 // This is an expression, and the value (5) is assigned to `y`
    }; // The entire block is an expression

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // Function call as an expression
    let z = add_two(7); // `add_two(7)` is an expression whose value is assigned to `z`

    println!("The value of z is: {}", z);
}

// Helper function used in the demo
fn add_two(num: i32) -> i32 {
    num + 2 // This is an expression. The function returns this value.
}

fn tuple_demo() {
    // Creating a tuple with different types
    let tup: (i32, f64, char) = (500, 6.4, 'a');

    // Destructuring the tuple into separate variables
    let (x, y, z) = tup;
    println!("Destructured tuple: x = {}, y = {}, z = {}", x, y, z);

    // Accessing tuple elements directly
    println!("Direct access: tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);
}

fn array_demo() {
    // Creating an array with the same type and fixed length
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Accessing array elements by index
    println!("First element of the array: {}", arr[0]);
    println!("Second element of the array: {}", arr[1]);

    // Iterating over the array
    println!("Iterating over the array:");
    for element in arr.iter() {
        println!("{}", element);
    }

    // Using the len method to get the array length
    println!("The array has {} elements", arr.len());
}

fn control_flow_demo() {
    // if-else if-else example
    let number = 6;
    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else {
        println!("{} is not divisible by 4 or 3", number);
    }

    // loop example
    let mut count = 0;
    loop {
        count += 1;
        println!("Loop iteration: {}", count);

        if count == 5 {
            break; // Exits the loop when count reaches 5
        }
    }

    // while loop example
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop example using a range
    println!("Counting up with a for loop:");
    for num in 1..4 {
        println!("{}", num);
    }

    // for loop example for iterating over an array
    let items = [10, 20, 30];
    println!("Iterating over an array:");
    for item in items.iter() {
        println!("Item value: {}", item);
    }
}

// Defining a function that takes two arguments and returns a value
fn add_numbers_function_demo(num1: i32, num2: i32) -> i32 {
    // The function adds the two arguments and returns the result.
    // The return type is specified after the '->' symbol.
    num1 + num2 // In Rust, the last expression in a function is implicitly returned.
}
fn scope_demo() {
    let outer_var = 10; // `outer_var` is in the outer scope of the function

    {
        let inner_var = 5; // `inner_var` is in the inner scope of this block

        // Both `outer_var` and `inner_var` are accessible here
        println!("Inner scope: outer_var = {}, inner_var = {}", outer_var, inner_var);
    }

    // Here, only `outer_var` is accessible; `inner_var` is out of scope
    println!("Outer scope: outer_var = {}", outer_var);

    // Trying to print `inner_var` here would result in a compile-time error
    // Uncommenting the line below will cause an error
    // println!("Outer scope: inner_var = {}", inner_var);
}

fn memory_owndership_demo() {
    // Stack memory allocation
    let x = 5; // `x` has a fixed size and is stored on the stack
    let y = x; // Copy of `x` is created and stored on the stack (because i32 implements Copy trait)

    println!("x: {}, y: {}", x, y); // Both `x` and `y` can be used as they are independent

    // Heap memory allocation
    let s1 = String::from("hello"); // `s1` is stored on the heap as its size can vary
    let s2 = s1; // `s1` is moved to `s2` - `s1` is no longer valid

    // Uncommenting the line below will result in a compile-time error
    // println!("s1: {}", s1); // This will cause an error because `s1`'s value was moved

    println!("s2: {}", s2);

    // Ownership and functions
    takes_ownership(s2); // `s2`'s value moves into the function...
    // ... and so is no longer valid here

    // Uncommenting the line below will result in a compile-time error
    // println!("s2: {}", s2); // This will cause an error because `s2` was moved

    let z = 5;
    makes_copy(z); // `z` would move into the function,
    println!("z: {}", z); // but i32 is Copy, so it's okay to still use `z` afterwards
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // `some_string` goes out of scope and `drop` is called. The backing memory is freed.
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    // `some_integer` goes out of scope. Nothing special happens as it's a copy.
}


fn clone_string_demo() {
    let original = String::from("Hello, Rust!"); // `original` is stored in the heap
    let clone = original.clone(); // Creates a full copy of `original` and stores it in a new memory location on the heap

    println!("Original String: {}", original);
    println!("Cloned String: {}", clone);

    // Both `original` and `clone` are valid and independent
    // Modifying one does not affect the other
    let modified_original = original + " Modified";
    let modified_clone = clone + " Modified";

    println!("Modified Original String: {}", modified_original);
    println!("Modified Cloned String: {}", modified_clone);
}

