pub fn add_two(x: i32) -> i32 {
    x + 2
}


pub fn add_negatives(x: i32, y: i32) -> i32 {
    if x >= 0 || y >= 0 {
        panic!("x and y must be less than 0, not {} and/or {}", x, y);
    }

    x + y
}


#[cfg(test)]
mod tests {
    use rand;
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn test_random_add_two() {
        for _ in 0..3 {
            let num = rand::random::<i32>();
            assert_eq!(num + 2, add_two(num));
        }
    }

    #[test]
    #[should_panic(expected = "x and y must be less than 0")]
    fn test_add_negatives() {
        add_negatives(-1, 32);
    }
}
