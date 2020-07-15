use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;


pub struct Cacher<T, K, V>
where
    T: Fn(K) -> V
{
    calculation: T,
    values: HashMap<K, V>
}


impl<T, K: Clone + Debug + Eq + Hash, V: Copy> Cacher<T, K, V>
where
    T: Fn(K) -> V
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }

    pub fn value(&mut self, k: K) -> V {
        match self.values.get(&k) {
            Some(&v) => v,
            None => {
                println!("Generating new result for {:?}", k);
                
                let v = (self.calculation)(k.clone());
                self.values.insert(k, v.clone());
                v
            }
        }
    }
}
