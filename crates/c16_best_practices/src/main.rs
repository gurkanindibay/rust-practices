//! Practice 16: Best Practices & Idioms
//! Demonstrates the Newtype Pattern, Builder Pattern, and Typestate Pattern (compile-time state verification).

// 1. Newtype pattern (type safety for primitives)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CustomerId(pub u64);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OrderId(pub u64);

// 2. Builder Pattern
#[derive(Debug, PartialEq)]
pub struct HttpClientConfig {
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub user_agent: String,
}

pub struct HttpClientConfigBuilder {
    timeout_ms: u64,
    max_retries: u32,
    user_agent: String,
}

impl HttpClientConfigBuilder {
    pub fn new() -> Self {
        Self {
            timeout_ms: 30_000,
            max_retries: 3,
            user_agent: "AzureRustClient/1.0".to_string(),
        }
    }

    pub fn timeout_ms(mut self, timeout: u64) -> Self {
        self.timeout_ms = timeout;
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn build(self) -> HttpClientConfig {
        HttpClientConfig {
            timeout_ms: self.timeout_ms,
            max_retries: self.max_retries,
            user_agent: self.user_agent,
        }
    }
}

impl Default for HttpClientConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 3. Typestate Pattern (compile-time state transitions)
pub struct Draft;
pub struct Published;

pub struct Article<State> {
    pub title: String,
    _state: std::marker::PhantomData<State>,
}

impl Article<Draft> {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            _state: std::marker::PhantomData,
        }
    }

    pub fn publish(self) -> Article<Published> {
        Article {
            title: self.title,
            _state: std::marker::PhantomData,
        }
    }
}

impl Article<Published> {
    pub fn read_url(&self) -> String {
        format!("https://knowledge.internal/articles/{}", self.title.to_lowercase().replace(' ', "-"))
    }
}

fn main() {
    println!("=== 16: Best Practices & Idioms ===");

    let cust_id = CustomerId(1001);
    let order_id = OrderId(5001);
    println!("Newtypes ensure IDs cannot be mixed: {cust_id:?}, {order_id:?}");

    let client_config = HttpClientConfigBuilder::new()
        .timeout_ms(5_000)
        .max_retries(5)
        .build();
    println!("Built client config: {client_config:?}");

    let draft = Article::new("Rust Best Practices in Cloud");
    // draft.read_url(); // Compile error: method does not exist on Draft!
    let published = draft.publish();
    println!("Published article URL: {}", published.read_url());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let cfg = HttpClientConfigBuilder::new()
            .timeout_ms(1000)
            .build();
        assert_eq!(cfg.timeout_ms, 1000);
        assert_eq!(cfg.max_retries, 3);
    }

    #[test]
    fn test_typestate_transition() {
        let draft = Article::new("Test Post");
        let published = draft.publish();
        assert_eq!(published.read_url(), "https://knowledge.internal/articles/test-post");
    }
}
