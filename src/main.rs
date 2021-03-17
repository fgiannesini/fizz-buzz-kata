fn main() {
}

fn fizzbuzz(value : i32) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz;

    #[test]
    fn should_get_fizz_if_value_is_3() {
        assert_eq!("fizz", fizzbuzz(3))
    }
}