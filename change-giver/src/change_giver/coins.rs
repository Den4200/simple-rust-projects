pub enum Coins {
    Quarter,
    Dime,
    Nickel,
    Penny
}


impl Coins {
    pub fn value(&self) -> f32 {
        match *self {
            Coins::Penny => 0.01,
            Coins::Nickel => 0.05,
            Coins::Dime => 0.1,
            Coins::Quarter => 0.25
        }
    }

    pub fn as_enum(&self) -> Coins {
        match *self {
            Coins::Quarter => Coins::Quarter,
            Coins::Dime => Coins::Dime,
            Coins::Nickel => Coins::Nickel,
            Coins::Penny => Coins::Penny
        }
    }

    pub fn as_string(&self) -> String {
        match *self {
            Coins::Quarter => String::from("Quarters"),
            Coins::Dime => String::from("Dimes"),
            Coins::Nickel => String::from("Nickels"),
            Coins::Penny => String::from("Pennies")
        }
    }

    pub fn as_vec() -> Vec<Coins> {
        vec![Coins::Quarter, Coins::Dime, Coins::Nickel, Coins::Penny]
    }
}
