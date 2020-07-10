pub fn add_two(x: i32) -> i32 {
    x + 2
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
}
