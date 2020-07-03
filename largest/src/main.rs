fn main() {
    let numbers = vec![34, 123, 73, 52, 8, 482, 21];
    println!("The largest number is {}", largest(&numbers));
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
