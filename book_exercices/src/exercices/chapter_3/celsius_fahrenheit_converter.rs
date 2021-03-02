pub fn fahrenheit_to_celsius(f_temp: i32) -> i32 {
    (f_temp - 32) * 5 / 9
}

pub fn celsius_to_fahrenheit(c_temp: i32) -> i32 {
    (c_temp * 9 / 5) + 32
}
