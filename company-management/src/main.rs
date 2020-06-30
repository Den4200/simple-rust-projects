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
            4. Remove Department
            5. Transfer employee to another department
            6. List people in a department
            7. List all people in all departments
            8. Exit
        "));

        let resp: u8 = menu
            .trim()
            .parse()
            .expect("You did not enter a number within the bounds.");

        match resp {
            1 => add_employee(&mut departments),
            2 => remove_employee(&mut departments),
            3 => create_department(&mut departments),
            4 => remove_department(&mut departments),
            5 => transfer_employee(&mut departments),
            6 => {
                let name = input("What is the department's name?");

                match departments.get(&name) {
                    None => println!("Department does not exist!"),
                    Some(department) => department.list_employees()
                }
            },
            7 => {
                for department in departments.values() {
                    department.list_employees();
                    println!();
                }
            },
            8 => {
                println!("Exiting program..");
                exit(0);
            },
            _ => println!("You did not enter a number between 1 and 6.\n")
        };
    };
}


fn add_employee(departments: &mut HashMap<String, Department>) {
    let full_name = input("What is their full name?");

    println!("\nDepartments:");
    for (d_name, dep) in departments.iter() {
        println!(" - {}", d_name);

        for emp in dep.employees.iter() {
            if emp.full_name == full_name {
                println!(
                    "{} is already in department {}! Please use option 3 instead, to transfer the employee.",
                    full_name,
                    emp.department_name
                );
                return;
            }
        }
    }
    println!();

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


fn remove_employee(departments: &mut HashMap<String, Department>) {
    let employee_name = input("What is the employee's name?");
    let department_name = input("What department is employee from?");

    for department in departments.values_mut() {
        if *department.name == department_name {
            department.remove_employee(&employee_name);
            println!("Removed {} successfully", employee_name);
            return;
        }
    }

    println!("Department does not exist!");
}


fn create_department(departments: &mut HashMap<String, Department>) {
    let name = input("What will the department's name be?");

    for dep_name in departments.keys() {
        if *dep_name == name {
            println!("The {} department already exists!", name);
            return;
        }
    }

    let department = Department {
        name: name.clone(),
        employees: Vec::new()
    };

    departments.insert(name.clone(), department);

    println!("The {} department has been created.", name)
}


fn remove_department(departments: &mut HashMap<String, Department>) {
    let department_name = input("What is the department's name?");
    let is_sure = input(
        indoc!("
            This will permanently remove all employees in this department.
            Transfer them first to another department if you do not want them to be deleted.
            Are you still sure you want to delete this department? (y/N)
        ")
    ).to_lowercase();

    if is_sure == "y" {
        departments.remove(&department_name);
        println!("The {} department has been removed.", department_name);
    } else {
        println!("Department deletion aborted.");
    }
}


fn transfer_employee(departments: &mut HashMap<String, Department>) {
    let employee_name = input("What is the employee's name?");
    let current_department_name = input("What is their current department's name?");
    let new_department_name = input("What department do you want to transfer this employee to?");

    match departments.get_mut(&current_department_name) {
        None => println!("Department does not exist!"),
        Some(department) => {
            for employee in department.employees.iter() {
                if *employee.full_name == employee_name {
                    department.remove_employee(&employee_name);

                    match departments.get_mut(&new_department_name) {
                        None => { println!("Department does not exist!"); return; }
                        Some(department) => {
                            department.add_employee(
                                Employee {
                                    full_name: employee_name.clone(),
                                    department_name: new_department_name.clone()
                                }
                            );
                            println!("{} was successfully transferred to the {} department.", employee_name, new_department_name);
                        }
                    }

                    return;
                }
            }
            println!("{} is not in the {} department!", employee_name, current_department_name);
        }
    }
}


fn input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    response.trim().to_string()
}
