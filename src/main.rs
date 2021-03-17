fn main() {
}

fn fizzbuzz(value: i32) -> String {
    let mut result = "".to_string();
    if value % 3 == 0 {
        result.push_str("fizz")
    }
    if value % 5 == 0 {
        result.push_str("buzz")
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz;

    #[test]
    fn should_get_fizz_if_value_is_3() {
        assert_eq!("fizz", fizzbuzz(3))
    }

    #[test]
    fn should_get_empty_if_value_is_1() {
        assert_eq!("", fizzbuzz(1))
    }

    #[test]
    fn should_get_empty_if_value_is_6() {
        assert_eq!("fizz", fizzbuzz(6))
    }

    #[test]
    fn should_get_buzz_if_value_is_5() {
        assert_eq!("buzz", fizzbuzz(5))
    }

    #[test]
    fn should_get_buzz_if_value_is_10() {
        assert_eq!("buzz", fizzbuzz(10))
    }

    #[test]
    fn should_get_fizzbuzz_if_value_is_15() {
        assert_eq!("fizzbuzz", fizzbuzz(15))
    }
}