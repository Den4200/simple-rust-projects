use std::collections::HashMap;
use std::io;


fn main() {
    println!("Input numbers one by one, pressing the Enter key after each one.");
    println!("Input the letter \"q\" when you want to stop.");

    let mut numbers: Vec<i32> = Vec::new();

    loop {
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line.");

        num = num.trim().to_string();

        if num == "q" {
            break;
        }

        let num: i32 = num
            .parse()
            .expect("You did not enter a number.");

        numbers.push(num);
    }
    
    println!("The mean is: {}", mean(&numbers));
    println!("The median is: {}", median(&numbers));
    println!("The mode is: {}", mode(&numbers));
}


fn mean(numbers: &Vec<i32>) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}


fn median(numbers: &Vec<i32>) -> f32 {
    let len = numbers.len();
    let half_len = (len / 2) as usize;

    if len % 2 == 1 {
        return numbers[half_len] as f32;
    }
    (numbers[half_len - 1] + numbers[half_len]) as f32 / 2.0
}


fn mode(numbers: &Vec<i32>) -> i32 {
    let mut num_map: HashMap<i32, u32> = HashMap::new();

    for num in numbers {
        let amt = num_map.entry(*num).or_insert(0);
        *amt += 1;
    }

    let mut max_value = 0;
    let mut key = 0;

    for (num, amt) in num_map {
        if amt > max_value {
            key = num;
            max_value = amt;
        }
    }
    key
}
