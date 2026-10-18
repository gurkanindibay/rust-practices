//! Practice 04: Structs, Enums & Pattern Matching
//! Demonstrates custom data types, impl methods, enums with payload, and Option/Result handling.

#[derive(Debug, PartialEq)]
pub struct ServiceNode {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub is_healthy: bool,
}

impl ServiceNode {
    pub fn new(name: impl Into<String>, ip: impl Into<String>, port: u16) -> Self {
        Self {
            name: name.into(),
            ip: ip.into(),
            port,
            is_healthy: true,
        }
    }

    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn mark_unhealthy(&mut self) {
        self.is_healthy = false;
    }
}

#[derive(Debug, PartialEq)]
pub enum CloudEvent {
    NodeProvisioned { id: u64, region: String },
    NodeHealthCheck { id: u64, healthy: bool },
    Shutdown,
}

impl CloudEvent {
    pub fn describe(&self) -> String {
        match self {
            CloudEvent::NodeProvisioned { id, region } => {
                format!("Node {id} provisioned in {region}")
            }
            CloudEvent::NodeHealthCheck { id, healthy } => {
                let status = if *healthy { "healthy" } else { "unhealthy" };
                format!("Node {id} reported {status}")
            }
            CloudEvent::Shutdown => "Cluster shutdown requested".to_string(),
        }
    }
}

fn main() {
    println!("=== 04: Structs, Enums & Pattern Matching ===");

    let mut node = ServiceNode::new("gateway-01", "10.0.0.5", 8080);
    println!("Node endpoint: {}", node.endpoint());
    node.mark_unhealthy();
    println!("Node status: {:?}", node);

    let event = CloudEvent::NodeProvisioned {
        id: 42,
        region: "westeurope".to_string(),
    };
    println!("Event description: {}", event.describe());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_node() {
        let node = ServiceNode::new("api", "127.0.0.1", 3000);
        assert_eq!(node.endpoint(), "127.0.0.1:3000");
        assert!(node.is_healthy);
    }

    #[test]
    fn test_cloud_event() {
        let event = CloudEvent::NodeHealthCheck { id: 1, healthy: true };
        assert_eq!(event.describe(), "Node 1 reported healthy");
    }
}
