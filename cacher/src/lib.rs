struct Cacher<T>
where
    T: Fn(u128) -> u128
{
    calculation: T,
    value: Option<u128>
}


impl<T> Cacher<T>
where
    T: Fn(u128) -> u128
{
    pub fn new(&self, calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    pub fn value(&mut self, value: u128) -> u128 {
        match self.value {
            Some(v) => v,
            None => {
                let v = self.calculation(value);
                self.value = Some(v);
                v
            }
        }
    }
}
