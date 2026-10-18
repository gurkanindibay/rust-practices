//! Practice 13: Testing & Benchmarking Library
//! Demonstrates unit tests, test assertions, panic testing, and test helpers.

pub fn sanitize_input(input: &str) -> String {
    input.trim().to_lowercase()
}

pub fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_input() {
        assert_eq!(sanitize_input("  HELLO Azure  "), "hello azure");
        assert_eq!(sanitize_input("Rust"), "rust");
    }

    #[test]
    fn test_divide_success() {
        let result = divide(10.0, 2.0).unwrap();
        assert!((result - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
    }
}
