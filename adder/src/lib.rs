pub fn add_two(x: i32) -> i32 {
    x + 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }
}
