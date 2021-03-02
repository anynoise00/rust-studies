pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x > y {
        x
    } else {
        y
    }
}
