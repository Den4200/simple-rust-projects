pub struct Employee {
    pub full_name: String,
    pub department_name: String
}


impl Employee {
    pub fn change_department(&mut self, department_name: String) {
        self.department_name = department_name;
    }
}
