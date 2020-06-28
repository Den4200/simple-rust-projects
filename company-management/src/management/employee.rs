use crate::management::department::Department;


pub struct Employee {
    pub id: u32,
    pub full_name: String,
    pub department: Department
}


impl Employee {
    pub fn change_department(&mut self, department: Department) {
        self.department.remove_employee(self.id);
        self.department = department;
    }
}
