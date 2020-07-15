use std::io;

use cacher::Cacher;


fn main() {
    let square = Cacher::new(|num| num.pow(2));

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

}


fn input(msg: &str) -> String {
    print!("{}", msg);

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .unwrap();


    answer
}
