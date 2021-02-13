fn main() {
    let y = {
        let x = 3;
        x + 1 // This line without a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons.
    };

    println!("The value of y is: {}", y);

    print_value(return_value_plus_two(y));
}

fn print_value(x: i32) {
    println!("The value of x is: {}", x);
}

fn return_value_plus_two(y: i32) -> i32 {
    let value = y;
    value + 2 // Putting an ";" at the end of line turns the expression into a statement
    // So "value + 2;" will not compile
}