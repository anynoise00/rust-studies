use std::collections::HashMap;

pub fn mean(list_of_integers: &Vec<i32>) -> f32 {
    if list_of_integers.len() <= 0 {
        println!("Operation failed: The vector passed has a length of 0 or less.");
        return 0.0;
    }

    let mut total: i32 = 0;
    for value in list_of_integers {
        total += value;
    }

    total as f32 / list_of_integers.len() as f32
}

pub fn median(list_of_integers: &Vec<i32>) -> i32 {
    if list_of_integers.len() <= 0 {
        println!("Operation failed: The vector passed has a length of 0 or less.");
        return 0;
    }

    let middle_element_index = list_of_integers.len() / 2;

    let mut new_list = list_of_integers.clone();
    new_list.sort();
    new_list[middle_element_index]
}

pub fn mode(list_of_integers: &Vec<i32>) -> i32 {
    if list_of_integers.len() <= 0 {
        println!("Operation failed: The vector passed has a length of 0 or less.");
        return 0;
    }

    let mut occurrences: HashMap<&i32, i32> = HashMap::new();

    for value in list_of_integers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let mut most_occurring_pair = (0, 0);
    for (key, value) in occurrences {
        if value > most_occurring_pair.1 {
            most_occurring_pair = (*key, value);
        }
    }

    most_occurring_pair.0
}
