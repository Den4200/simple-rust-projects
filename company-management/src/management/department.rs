use crate::management::employee::Employee;


#[derive(Debug)]
pub struct Department {
    pub name: String,
    pub employees: Vec<Employee>
}


impl Department {

    pub fn add_employee(&mut self, employee: Employee) {
        self.employees.push(employee);
        self.employees.sort();
    }

    pub fn remove_employee(&mut self, employee_name: &String) {
        match self.employees.iter().position(|x| *x.full_name == *employee_name) {
            None => println!("Employee is not in this department!"),
            Some(index) => { self.employees.remove(index); }
        }
    }

    pub fn list_employees(&self) {
        println!("{} Department:", self.name);

        for employee in self.employees.iter() {
            println!(" - {}", employee.full_name);
        }
    }
}
