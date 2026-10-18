//! Practice 02: Ownership & Borrowing
//! Demonstrates move semantics, Copy vs Clone, immutable/mutable borrowing, and slices.

fn main() {
    println!("=== 02: Ownership & Borrowing ===");

    // Move semantics with heap data
    let s1 = String::from("Rust Systems");
    let s2 = s1; // Ownership moved to s2; s1 is now invalid
    println!("Moved string s2: {s2}");

    // Clone for deep copy
    let s3 = s2.clone();
    println!("Cloned s3: {s3}, s2 still valid: {s2}");

    // Copy semantics for stack types
    let a = 42;
    let b = a; // i32 implements Copy
    println!("Stack copy: a = {a}, b = {b}");

    // Borrowing (references)
    let len = calculate_length(&s2);
    println!("Length of '{s2}' is {len}");

    // Mutable borrowing
    let mut target = String::from("Cloud");
    append_platform(&mut target);
    println!("Mutated target: {target}");

    // Slices
    let text = String::from("Hello Azure Rust");
    let first = first_word(&text);
    println!("First word slice: '{first}'");
}

pub fn calculate_length(s: &str) -> usize {
    s.len()
}

pub fn append_platform(s: &mut String) {
    s.push_str(" Native");
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        assert_eq!(calculate_length("hello"), 5);
    }

    #[test]
    fn test_append_platform() {
        let mut s = String::from("Cloud");
        append_platform(&mut s);
        assert_eq!(s, "Cloud Native");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("Azure Cloud"), "Azure");
        assert_eq!(first_word("SingleWord"), "SingleWord");
    }
}
