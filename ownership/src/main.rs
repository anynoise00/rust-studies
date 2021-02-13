fn main() { // Start of the scope
    let string_literal = "hello"; // Isn't mutable
    let mut string = String::from(string_literal);

    string.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", string); // This will print `hello, world!`

    let s1 = String::from("heap");
    let s2 = takes_and_gives_back(s1); // This invalidates s1, if s1 has some data on the heap
    let s3 = s2.clone(); // This doesn't invalidates s2

    println!("s3 = {}", s3);
    
    takes_ownership(s3); // Invalidates s3, because a variable that stores data in the heap
    // doesn't have the Copy trait
    
    let x = 5;
    make_copy(x); // x would move into the function,
                  // but i32 is Copy, so it’s okay to still
                  // use x afterward

} // All variables are freed from the memory when they go out of scope

fn takes_ownership(x: String) {
    println!("x = {}", x);
}

fn make_copy(x: i32) {
    println!("x = {}", x);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string is returned and moves out to the calling function
}