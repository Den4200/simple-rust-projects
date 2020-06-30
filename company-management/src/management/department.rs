use crate::management::employee::Employee;


pub struct Department {
    pub name: String,
    pub employees: Vec<Employee>
}


impl Department {
    pub fn add_employee(&mut self, employee: Employee) {
        self.employees.push(employee);
    }

    pub fn remove_employee(&mut self, employee_name: &String) {
        let index = self.employees.iter().position(|x| *x.full_name == *employee_name).unwrap();
        self.employees.remove(index);
    }

    pub fn list_employees(&self) {
        println!("{} Department:", self.name);

        for employee in self.employees.iter() {
            println!("    {}", employee.full_name);
        }
    }
}
