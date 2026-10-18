//! Practice 05: Traits & Generics
//! Demonstrates trait definitions, trait bounds, default implementations, and dynamic dispatch (trait objects).

pub trait HealthCheckable {
    fn check_health(&self) -> bool;
    fn health_summary(&self) -> String {
        let status = if self.check_health() { "UP" } else { "DOWN" };
        format!("Health Status: {status}")
    }
}

pub struct DatabaseConnection {
    pub is_connected: bool,
}

impl HealthCheckable for DatabaseConnection {
    fn check_health(&self) -> bool {
        self.is_connected
    }
}

pub struct StorageClient {
    pub ping_ms: u64,
}

impl HealthCheckable for StorageClient {
    fn check_health(&self) -> bool {
        self.ping_ms < 500
    }
}

// Static dispatch using Generics
pub fn inspect_static<T: HealthCheckable>(resource: &T) -> String {
    resource.health_summary()
}

// Dynamic dispatch using Trait Objects
pub fn inspect_all_dynamic(resources: &[Box<dyn HealthCheckable>]) -> Vec<String> {
    resources.iter().map(|r| r.health_summary()).collect()
}

fn main() {
    println!("=== 05: Traits & Generics ===");

    let db = DatabaseConnection { is_connected: true };
    let storage = StorageClient { ping_ms: 120 };

    println!("DB: {}", inspect_static(&db));
    println!("Storage: {}", inspect_static(&storage));

    let cluster: Vec<Box<dyn HealthCheckable>> = vec![
        Box::new(DatabaseConnection { is_connected: true }),
        Box::new(StorageClient { ping_ms: 800 }),
    ];

    let summaries = inspect_all_dynamic(&cluster);
    for s in summaries {
        println!("Cluster resource: {s}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_checks() {
        let db = DatabaseConnection { is_connected: true };
        assert!(db.check_health());
        assert_eq!(inspect_static(&db), "Health Status: UP");

        let dead_storage = StorageClient { ping_ms: 600 };
        assert!(!dead_storage.check_health());
        assert_eq!(inspect_static(&dead_storage), "Health Status: DOWN");
    }
}
