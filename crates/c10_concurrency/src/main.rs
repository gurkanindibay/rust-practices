//! Practice 10: Concurrency
//! Demonstrates thread spawning, message passing with channels, and thread-scoped concurrency.

use std::sync::mpsc;
use std::thread;

pub fn calculate_parallel_sum(chunks: Vec<Vec<i64>>) -> i64 {
    let (tx, rx) = mpsc::channel();

    for chunk in chunks {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let sum: i64 = chunk.into_iter().sum();
            tx_clone.send(sum).unwrap();
        });
    }
    drop(tx); // Close original transmitter

    rx.into_iter().sum()
}

fn main() {
    println!("=== 10: Concurrency ===");

    // 1. Thread scope (Rust 1.63+)
    let numbers = [1, 2, 3, 4, 5];
    let mut results = vec![0; numbers.len()];

    thread::scope(|s| {
        for (i, res) in results.iter_mut().enumerate() {
            let val = numbers[i];
            s.spawn(move || {
                *res = val * 10;
            });
        }
    });

    println!("Thread scoped results: {:?}", results);

    // 2. Parallel chunk summation with channels
    let chunks = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let total = calculate_parallel_sum(chunks);
    println!("Parallel chunks total sum: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum() {
        let chunks = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8]];
        assert_eq!(calculate_parallel_sum(chunks), 36);
    }
}
