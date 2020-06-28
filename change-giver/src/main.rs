mod change_giver;

use std::io;


fn main() {
    println!("Please enter an amount of money: ");

    let mut amt = String::new();

    io::stdin()
        .read_line(&mut amt)
        .expect("Failed to read line.");

    let mut amt: f32 = amt
        .trim()
        .parse()
        .expect("Input was not a float.");

    let change = change_giver::get_change(&mut amt);

    println!();
    for (coin, amt) in change {
        println!("{} {}", amt, coin.as_string())
    }
}
