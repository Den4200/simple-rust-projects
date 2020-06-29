mod management;

use std::collections::HashMap;
use std::io;
use std::process::exit;

use indoc::indoc;

use management::{Department, Employee};


fn main() {
    let mut departments: HashMap<String, Department> = HashMap::new();

    loop {
        let menu = input(indoc!("
            1. Add Employee
            2. Remove Employee
            3. Create Department
            4. Delete Department
            3. Transfer employee to another department
            4. List people in a department
            5. List all people in all departments
            6. Exit
        "));

        let resp: u8 = menu
            .trim()
            .parse()
            .expect("You did not enter a number within the bounds.");

        match resp {
            1 => add_employee(&mut departments),
            3 => create_department(&mut departments),
            6 => {
                println!("Exiting program..");
                exit(0);
            },
            _ => println!("You did not enter a number between 1 and 6.\n")
        };
    };
}


fn add_employee(departments: &mut HashMap<String, Department>) {
    let full_name = input("What is their full name?");

    println!();
    for d_name in departments.keys() {
        println!("{}", d_name);
    }

    let dep = input("What department will they be added to?");

    match departments.get_mut(&dep) {
        None => println!("Department does not exist!"),
        Some(department) => {
            department.add_employee(
                Employee {
                    full_name: full_name.clone(),
                    department_name: department.name.clone()
                }
            );
            println!("{} was added to the {} department.", full_name, department.name);
        }
    };
}


fn create_department(departments: &mut HashMap<String, Department>) {
    let name = input("What will the department's name be?");
    let department = Department {
        name: name.clone(),
        employees: Vec::new()
    };

    departments.insert(name.clone(), department);

    println!("The {} department has been created.", name)
}


fn input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    response.trim().to_string()
}
