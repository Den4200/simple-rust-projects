use std::io;


fn main() {
    println!("Enter a number: ");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Incorrect input!");

    let number: u32 = number
        .trim()
        .parse()
        .expect("Please enter a number!");

    // Print a newline to space things out
    println!();
    fizzbuzz(number);
}

fn fizzbuzz(n: u32) {
    for x in 1..n+1 {
        if x % 3 != 0 && x % 5 != 0 {
            println!("{}", x);
        } else if x % 3 == 0 && x % 5 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        }
    }
}
