use std::io;

use cacher::Cacher;


fn main() {
    let mut square = Cacher::new(|num: u128| num.pow(2));

    let mut inputs = Vec::new();

    println!("Enter an integer (\"q\" to stop):");
    loop {
        let answer = input("");

        if answer.trim() == "q" {
            break;
        }

        let answer: u128 = answer
            .trim()
            .parse()
            .unwrap();

        inputs.push(answer);
    }

    for inp in inputs {
        println!("The square of {} is {}", inp, square.value(inp));
    }
}


fn input(msg: &str) -> String {
    print!("{}", msg);

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .unwrap();


    answer
}
