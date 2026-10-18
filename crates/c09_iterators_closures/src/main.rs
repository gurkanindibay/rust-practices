//! Practice 09: Iterators & Closures
//! Demonstrates closures, iterator adaptors, chaining, and custom iterator implementations.

pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(current)
    }
}

pub fn sum_of_even_squares(limit: u32) -> u32 {
    (1..=limit)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum()
}

fn main() {
    println!("=== 09: Iterators & Closures ===");

    // Closures
    let multiplier = 3;
    let multiply = |x: i32| x * multiplier;
    println!("Closure result: 5 * 3 = {}", multiply(5));

    // Custom iterator: first 8 Fibonacci numbers
    let fib_series: Vec<u64> = Fibonacci::new().take(8).collect();
    println!("First 8 Fibonacci numbers: {:?}", fib_series);

    let even_sq_sum = sum_of_even_squares(10);
    println!("Sum of even squares up to 10: {even_sq_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = Fibonacci::new().take(6).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5]);
    }

    #[test]
    fn test_sum_of_even_squares() {
        // 2^2 + 4^2 = 4 + 16 = 20
        assert_eq!(sum_of_even_squares(4), 20);
    }
}
