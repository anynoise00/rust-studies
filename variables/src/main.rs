const GLOBAL_SCOPE_CONST_X: u32 = 10; // A constant is immutable and can be used in a global scope

fn main() {
    let mut x = GLOBAL_SCOPE_CONST_X; // Mutable variable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y: u32 = 20; // This variable isn't mutable, but can be redeclared
    println!("The value of y is: {}", y);
    let y = y + 5; // Shadowing an already declared variable
    println!("The value of y is: {}", y);
    let y: String = y.to_string(); // It's the same as redeclaring the variable, with the same name, but a different type
    println!("The value of y is: {}", y);
}