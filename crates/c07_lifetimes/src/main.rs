//! Practice 07: Lifetimes Deep Dive
//! Demonstrates explicit lifetime parameters, struct references with lifetimes, and static strings.

// Function with explicit lifetime annotation: returned reference lives as long as the shorter input
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct holding references with lifetime parameter
#[derive(Debug)]
pub struct HighlightedSpan<'a> {
    pub source_text: &'a str,
    pub title: &'a str,
}

impl<'a> HighlightedSpan<'a> {
    pub fn first_sentence(&self) -> &'a str {
        self.source_text
            .split('.')
            .next()
            .unwrap_or(self.source_text)
    }
}

// Static lifetime string
pub fn get_app_version() -> &'static str {
    "1.0.0-azure-rust"
}

fn main() {
    println!("=== 07: Lifetimes Deep Dive ===");

    let str1 = String::from("Kubernetes Service");
    let str2 = "App Service";

    let result = longest(&str1, str2);
    println!("Longest string is: '{result}'");

    let doc = String::from("Rust provides memory safety without a GC. Lifetimes prevent dangling pointers.");
    let span = HighlightedSpan {
        source_text: &doc,
        title: "Rust Architecture",
    };
    println!("Highlight span title: {}", span.title);
    println!("First sentence: '{}'", span.first_sentence());
    println!("Static version: {}", get_app_version());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let a = "short";
        let b = "longer string";
        assert_eq!(longest(a, b), "longer string");
        assert_eq!(longest(b, a), "longer string");
    }

    #[test]
    fn test_highlighted_span() {
        let content = "First sentence. Second sentence.";
        let span = HighlightedSpan {
            source_text: content,
            title: "Test",
        };
        assert_eq!(span.first_sentence(), "First sentence");
    }
}
