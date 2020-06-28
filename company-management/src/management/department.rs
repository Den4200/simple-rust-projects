use crate::management::employee::Employee;


pub struct Department {
    pub name: String,
    pub employees: Vec<Employee>
}


impl Department {
    pub fn add_employee(&mut self, employee: Employee) {
        self.employees.push(employee);
    }

    pub fn remove_employee(&mut self, employee_id: u32) {
        let index = self.employees.iter().position(|x| x.id == employee_id).unwrap();
        self.employees.remove(index);
    }
}
