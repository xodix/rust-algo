fn main() {}

fn zeros_in_factorial(n: u64) -> u64 {
    n / 10
}

fn factorial(i: u64) -> u64 {
    (1..(i + 1))
        .reduce(|a, b| a * b)
        .expect("why doesn't that work")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_factorial() {
        use super::factorial;
        assert_eq!(factorial(1_000_000_000), 1_000_000_000);
    }
}
