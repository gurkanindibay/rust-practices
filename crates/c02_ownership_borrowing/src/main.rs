//! Practice 02: Ownership & Borrowing
//!
//! A comprehensive showcase matching every section and concept in
//! `programming-languages/rust/lessons/02-ownership-borrowing.md`:
//!
//! 1. Why Ownership & RAII (Scope exit, Drop trait)
//! 2. Ownership Rules & Invariants
//! 3. Move Semantics (Variables & Functions)
//! 4. Clone vs. Copy (Stack types vs. Heap deep clone)
//! 5. Borrowing & References (&T immutable vs. &mut T mutable)
//! 6. The Borrow Checker & Non-Lexical Lifetimes (NLL)
//! 7. Slices as Fat Pointers (Strings, Arrays, Vectors)
//! 8. Lifetimes (Elision & Explicit Lifetime Annotations)
//! 9. Common Pitfalls & Solutions (Dangling references, Closures)

// ============================================================================
// 1. Why Ownership & RAII: Scope, Single Owner & Drop Trait
// ============================================================================

/// Custom resource to visibly log when destructor (Drop) is executed.
pub struct ResourceTracker {
    pub name: String,
}

impl ResourceTracker {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        println!("  [ResourceTracker] '{name}' allocated on heap.");
        Self { name }
    }
}

impl Drop for ResourceTracker {
    fn drop(&mut self) {
        println!("  [ResourceTracker] '{name}' dropped automatically! (RAII destructor called)", name = self.name);
    }
}

pub fn demo_ownership_and_raii() {
    println!("\n--- 1 & 2. Ownership Rules & RAII (Drop Mechanics) ---");
    println!("Rule 1: Each value has exactly one owner.");
    println!("Rule 2: When the owner goes out of scope, the value is dropped.");
    println!("Rule 3: There can only be one owner at a time.");

    {
        println!("  Entering inner scope...");
        let _resource_a = ResourceTracker::new("Buffer-Alpha");
        let _resource_b = ResourceTracker::new("Buffer-Beta");
        println!("  Working with resources inside scope...");
        println!("  Leaving inner scope now:");
    }
    println!("  Outside scope: Both buffers have been safely and deterministically deallocated!");
}

// ============================================================================
// 3. Move Semantics (Assignment, Functions & Returning Ownership)
// ============================================================================

pub fn take_ownership(s: String) {
    println!("  [take_ownership] Received: \"{s}\". Ownership transferred here; will drop at end of function.");
}

pub fn give_ownership() -> String {
    let fresh = String::from("Hello from producer");
    println!("  [give_ownership] Created fresh String; moving ownership out to caller.");
    fresh
}

pub fn take_and_give_back(mut s: String) -> String {
    s.push_str(" (enriched by take_and_give_back)");
    println!("  [take_and_give_back] Modified and returning ownership to caller.");
    s
}

pub fn demo_move_semantics() {
    println!("\n--- 3. Move Semantics ---");

    // Move between variables
    let s1 = String::from("Rust Systems");
    let s2 = s1; // s1 is MOVED into s2. s1 is uninitialized / invalidated at compile-time.
    println!("  Moved: s2 has ownership: \"{s2}\"");
    // println!("{s1}"); // ❌ Compile error: borrow of moved value: `s1`

    // Moving into a function
    let to_send = String::from("Transient payload");
    take_ownership(to_send);
    // println!("{to_send}"); // ❌ Compile error: value used after move

    // Receiving ownership from function
    let received = give_ownership();
    println!("  Received from give_ownership: \"{received}\"");

    // Passing in and receiving back
    let enriched = take_and_give_back(received);
    println!("  Enriched result: \"{enriched}\"");
}

// ============================================================================
// 4. Clone vs Copy
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

pub fn demo_clone_vs_copy() {
    println!("\n--- 4. Clone vs Copy ---");

    // Copy types: Stack-only, bitwise copy
    println!("* Copy Semantics (Stack-only, implicit bitwise duplication):");
    let num1: i32 = 42;
    let num2 = num1; // Copied!
    println!("  i32: num1 = {num1}, num2 = {num2} (both fully valid)");

    let p1 = Point2D { x: 10, y: 20 };
    let p2 = p1; // Struct implements Copy because all fields are Copy
    println!("  Point2D: p1 = {p1:?}, p2 = {p2:?} (no move occurred)");

    let tuple_a = (100, true, '🦀');
    let tuple_b = tuple_a; // Tuples composed of Copy types also implement Copy
    println!("  Tuple: a = {tuple_a:?}, b = {tuple_b:?}");

    // Clone: Heap-allocated, explicit deep copy
    println!("* Clone Semantics (Explicit heap deep copy):");
    let original = String::from("Canonical Config");
    let duplicated = original.clone(); // Separate heap allocation
    println!("  String: original = \"{original}\", cloned = \"{duplicated}\" (both owners valid)");
}

// ============================================================================
// 5. Borrowing & References
// ============================================================================

pub fn calculate_length(s: &str) -> usize {
    // s is borrowed; original caller retains ownership
    s.len()
}

pub fn append_platform(s: &mut String) {
    // Exclusive mutable access allows in-place mutation
    s.push_str(" Platform");
}

pub fn demo_borrowing_and_references() {
    println!("\n--- 5. Borrowing & References ---");

    // Immutable borrowing
    let base = String::from("Azure Cloud");
    let len = calculate_length(&base);
    println!("  Immutable borrow: \"{base}\" has length {len}. (base still owned and valid!)");

    // Mutable borrowing
    let mut target = String::from("Container");
    println!("  Before mutation: \"{target}\"");
    append_platform(&mut target);
    println!("  After mutable borrow: \"{target}\"");
}

// ============================================================================
// 6. The Borrow Checker & Non-Lexical Lifetimes (NLL)
// ============================================================================

pub fn demo_borrow_checker_and_nll() {
    println!("\n--- 6. The Borrow Checker & Non-Lexical Lifetimes (NLL) ---");
    println!("Core Invariant: Aliasing XOR Mutability (Many &T OR exactly one &mut T, never both)");

    let mut numbers = vec![10, 20, 30];

    // Multiple immutable borrows coexist
    let ref1 = &numbers;
    let ref2 = &numbers;
    println!("  Shared borrow ref1: {:?}, ref2: {:?}", ref1, ref2);
    // Non-Lexical Lifetimes: ref1 and ref2 are no longer used past this line.

    // Therefore, an exclusive mutable borrow is safely permitted right here:
    let mut_ref = &mut numbers;
    mut_ref.push(40);
    println!("  Exclusive mutable borrow modified vector: {:?}", mut_ref);

    // Vector re-allocation safety:
    // Demonstrating why Rust forbids pushing while holding a reference to an element:
    // let first = &numbers[0];
    // numbers.push(50); // ❌ Compile error: cannot borrow `numbers` as mutable because it is also borrowed as immutable
    // println!("{first}"); // If allowed, `first` would point to invalidated memory if the vector reallocated its buffer!
    println!("  Memory safety guaranteed: cannot mutate collection while holding element reference.");
}

// ============================================================================
// 7. Slices (Fat Pointers)
// ============================================================================

pub fn inspect_fat_pointer(name: &str, slice: &[i32]) {
    println!(
        "  Slice fat pointer '{name}': ptr = {:p}, len = {}",
        slice.as_ptr(),
        slice.len()
    );
}

pub fn demo_slices() {
    println!("\n--- 7. Slices (Fat Pointers: Data Pointer + Length) ---");

    // String slices (&str)
    let message = String::from("Hello Azure Cloud");
    let hello: &str = &message[0..5];
    let cloud: &str = &message[12..17];
    println!("  Original String: \"{message}\"");
    println!("  String slice 1: \"{hello}\"");
    println!("  String slice 2: \"{cloud}\"");

    // Array and Vector slices
    let array = [1, 2, 3, 4, 5, 6, 7, 8];
    let arr_sub: &[i32] = &array[2..6]; // [3, 4, 5, 6]
    println!("  Array: {array:?}, Sub-slice [2..6]: {arr_sub:?}");
    inspect_fat_pointer("arr_sub", arr_sub);

    let vec_data = vec![100, 200, 300, 400];
    let vec_sub: &[i32] = &vec_data[1..3]; // [200, 300]
    println!("  Vector: {vec_data:?}, Sub-slice [1..3]: {vec_sub:?}");
    inspect_fat_pointer("vec_sub", vec_sub);
}

// ============================================================================
// 8. Lifetimes — Introduction
// ============================================================================

/// Lifetime elision: The compiler infers that the output reference
/// is tied to the single input reference `s`.
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

/// Explicit lifetime annotation: Tells the compiler that the returned reference
/// will live at least as long as the intersection of lifetimes of `x` and `y`.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn demo_lifetimes() {
    println!("\n--- 8. Lifetimes (Inferred & Explicit) ---");

    // Lifetime elision
    let headline = "Kubernetes Microservices Architecture";
    let first = first_word(headline);
    println!("  Elided lifetime - first word of \"{headline}\": \"{first}\"");

    // Explicit lifetime parameter
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("  Explicit lifetime 'a - longest string is: \"{result}\"");
    }
}

// ============================================================================
// 9. Common Pitfalls & Solutions
// ============================================================================

/// Solution to Pitfall 1 (Dangling Reference): Return an owned value, not a local reference.
pub fn create_valid_string() -> String {
    // Cannot return `&String` here because local var is dropped at scope end.
    // Returning `String` moves ownership cleanly to the caller.
    String::from("Safe Owned Heap String")
}

pub fn demo_common_pitfalls() {
    println!("\n--- 9. Common Pitfalls & Proven Solutions ---");

    // Pitfall 1: Dangling references
    println!("* Pitfall 1: Returning reference to a local stack variable (Dangling Pointer)");
    println!("  ❌ fn bad() -> &String {{ let s = String::new(); &s }} // Compile error!");
    let safe_owned = create_valid_string();
    println!("  ✅ Solution: Return owned `String` instead: \"{safe_owned}\"");

    // Pitfall 2: Closures capturing by move vs. borrow
    println!("* Pitfall 2: Closure capture and `move` keyword");
    let greeting = String::from("Welcome");
    
    // Non-move closure borrows `greeting` by reference
    let borrow_closure = || println!("  [Borrow Closure] Greeting: {greeting}");
    borrow_closure();
    println!("  After borrow closure, greeting is still usable: \"{greeting}\"");

    // `move` closure takes full ownership of `greeting`
    let move_closure = move || {
        println!("  [Move Closure] Took ownership of greeting: \"{greeting}\"");
        // `greeting` is dropped when this closure is dropped
    };
    move_closure();
    // println!("{greeting}"); // ❌ Compile error: value borrowed here after move
    println!("  After move closure, `greeting` cannot be used in outer scope because ownership moved.");
}

// ============================================================================
// Summary & Runner
// ============================================================================

fn main() {
    println!("============================================================");
    println!("🦀 Rust Learning: 02 - Ownership & Borrowing Comprehensive Demo");
    println!("============================================================");

    demo_ownership_and_raii();
    demo_move_semantics();
    demo_clone_vs_copy();
    demo_borrowing_and_references();
    demo_borrow_checker_and_nll();
    demo_slices();
    demo_lifetimes();
    demo_common_pitfalls();

    println!("\n============================================================");
    println!("✅ All Ownership & Borrowing showcases executed successfully!");
    println!("============================================================");
}

// ============================================================================
// Unit Tests Validating Every Concept
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_and_ownership_exchange() {
        let original = give_ownership();
        assert_eq!(original, "Hello from producer");

        let enriched = take_and_give_back(original);
        assert!(enriched.starts_with("Hello from producer"));
        assert!(enriched.contains("enriched"));
    }

    #[test]
    fn test_copy_semantics() {
        let p1 = Point2D { x: 5, y: 15 };
        let p2 = p1; // Copy
        assert_eq!(p1, p2);
        assert_eq!(p1.x, 5); // p1 is still valid
    }

    #[test]
    fn test_clone_semantics() {
        let s1 = String::from("Hello Clone");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
        assert_ne!(s1.as_ptr(), s2.as_ptr()); // Different heap locations
    }

    #[test]
    fn test_immutable_borrow_and_length() {
        let msg = String::from("Rustacean");
        let len = calculate_length(&msg);
        assert_eq!(len, 9);
        assert_eq!(msg, "Rustacean"); // Not moved
    }

    #[test]
    fn test_mutable_borrow_mutation() {
        let mut target = String::from("Azure");
        append_platform(&mut target);
        assert_eq!(target, "Azure Platform");
    }

    #[test]
    fn test_slices() {
        let s = String::from("Distributed Systems");
        let word1 = &s[0..11];
        let word2 = &s[12..19];
        assert_eq!(word1, "Distributed");
        assert_eq!(word2, "Systems");

        let arr = [10, 20, 30, 40, 50];
        let slice = &arr[1..4];
        assert_eq!(slice, &[20, 30, 40]);
    }

    #[test]
    fn test_lifetime_functions() {
        assert_eq!(first_word("Cloud Native Rust"), "Cloud");
        assert_eq!(first_word("SingleToken"), "SingleToken");

        let s1 = "short";
        let s2 = "longer string";
        assert_eq!(longest(s1, s2), "longer string");
    }

    #[test]
    fn test_closure_move_behavior() {
        let text = String::from("captured");
        let closure = move || -> String { text };
        assert_eq!(closure(), "captured");
    }
}
