use std::io;


fn main() {
    println!("Enter a number.");

    let mut nth_number = String::new();

    io::stdin()
        .read_line(&mut nth_number)
        .expect("Error on input");

    let nth_number: u64 = nth_number.trim().parse().expect("You must enter a positive number.");

    fibonacci(nth_number);
}

fn fibonacci(n: u64) {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128;

    for _ in 2..n+1 {
        c = a + b;
        a = b;
        b = c;

        println!("Fibonacci number: {}", c);
    }
}
