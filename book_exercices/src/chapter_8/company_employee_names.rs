use std::collections::HashMap;

pub struct Company {
    name: String,
    employees: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new(name: String) -> Company {
        Company {
            name,
            employees: HashMap::new(),
        }
    }

    pub fn add_employee_to_department(&mut self, employee: String, department: String) {
        let department_vec = self.employees.entry(department).or_insert(Vec::new());
        department_vec.push(employee);
        department_vec.sort();
    }

    pub fn list_all_employees_in_department(&mut self, department: String) {
        if self.employees.contains_key(&department) {
            let department_vec = &self.employees[&department];
            let mut employee_list = String::new();
            for employee in department_vec.iter() {
                employee_list = employee_list + &format!("- {}\n", employee);
            }
            println!(
                "The employees from company {} in the {} department are:\n\
                {}",
                self.name, department, employee_list
            );
        } else {
            println!("The company doesn't have a/an {} department.", department);
        }
    }

    pub fn list_all_employees(&self) {
        let mut keys: Vec<&String> = Vec::new();

        for key in self.employees.keys() {
            keys.push(key);
        }

        keys.sort();

        let mut employee_list = String::new();

        for key in keys {
            employee_list = employee_list + &format!("{}\n", key);
            for employee in &self.employees[key] {
                employee_list = employee_list + &format!("- {}\n", employee);
            }
        }

        println!(
            "The list of employees from company {} is:\n\
            {}",
            self.name, employee_list
        );
    }
}
