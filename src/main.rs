fn main() {
}

fn fizzbuzz(value : i32) -> String {
    if value % 3 == 0 {
        "fizz".to_string()
    } else if value == 5 || value == 10 {
        "buzz".to_string()
    } else {
        "".to_string()
    }

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
}