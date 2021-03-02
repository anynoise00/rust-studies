mod exercices;

use exercices::chapter_10::lifetimes::longest;
use exercices::chapter_8::{
    company_employee_names::Company, mean_median_mode, pig_latin::pig_latin,
};

fn main() {
    let int_list = vec![-1, -1, -1, 1, 1, 1, 1, 2, 3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5];
    println!(
        "Mean: {}, Median: {}, Mode: {}",
        mean_median_mode::mean(&int_list),
        mean_median_mode::median(&int_list),
        mean_median_mode::mode(&int_list),
    );

    let word1 = "apple".to_string();
    let word2 = "first".to_string();
    println!(
        "\n'{}' in pig latin is '{}', and '{}' in pig latin is '{}'\n",
        word1,
        pig_latin(&word1),
        word2,
        pig_latin(&word2),
    );

    let mut default_company = Company::new("Default".to_string());
    default_company.add_employee_to_department("Mathew".to_string(), "Technology".to_string());
    default_company.add_employee_to_department("Barbara".to_string(), "Infrastructure".to_string());
    default_company.add_employee_to_department("Lucas".to_string(), "Technology".to_string());

    default_company.list_all_employees_in_department("Non-existent".to_string());
    default_company.list_all_employees_in_department("Technology".to_string());
    default_company.list_all_employees();

    // Try designing more experiments that vary the values and lifetimes
    // of the references passed in to the longest function and how the returned
    // reference is used. Make hypotheses about whether or not your experiments
    // will pass the borrow checker before you compile; then check to see if you’re right!

    // will compile
    let x = "Biiiiiiig!";
    {
        let y = "Small";
        longest(&x, &y);
    }

    // will not compile
    //let w = "Biiiiiiiiiiiiig!";
    //{
    //    let z = "Small";
    //}
    //longest(&w, &z);
}
