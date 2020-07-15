use std::collections::HashMap;


pub struct Cacher<T>
where
    T: Fn(u128) -> u128
{
    calculation: T,
    values: HashMap<u128, u128>
}


impl<T> Cacher<T>
where
    T: Fn(u128) -> u128
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }

    pub fn value(&mut self, value: u128) -> u128 {
        match self.values.get(&value) {
            Some(&v) => v,
            None => {
                println!("Generating new result for {}", value);
                
                let v = (self.calculation)(value);
                self.values.insert(value, v);
                v
            }
        }
    }
}
