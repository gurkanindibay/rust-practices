//! Practice 11: Async/Await & Tokio
//! Demonstrates the Tokio runtime, async functions, task spawning, async channels, and select!.

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

pub async fn fetch_mock_data(id: u32, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Payload-{id}")
}

#[tokio::main]
async fn main() {
    println!("=== 11: Async/Await & Tokio ===");

    // Spawn concurrent async tasks
    let task1 = tokio::spawn(fetch_mock_data(1, 100));
    let task2 = tokio::spawn(fetch_mock_data(2, 50));

    let (res1, res2) = tokio::join!(task1, task2);
    println!("Task 1: {}", res1.unwrap());
    println!("Task 2: {}", res2.unwrap());

    // Tokio mpsc channel
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(format!("Event-{i}")).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("Received async message: {msg}");
    }

    // Tokio select! macro
    tokio::select! {
        res = fetch_mock_data(42, 30) => {
            println!("Select winner: {res}");
        }
        _ = sleep(Duration::from_millis(100)) => {
            println!("Timed out!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_mock_data() {
        let result = fetch_mock_data(99, 10).await;
        assert_eq!(result, "Payload-99");
    }
}
