//! Practice 12: Unsafe Rust
//! Demonstrates raw pointers, unsafe blocks, and wrapping unsafe code in safe abstractions.

/// Safe abstraction over unsafe split_at_mut logic
pub fn safe_split_at_mut<T>(values: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len, "Mid index out of bounds");

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    println!("=== 12: Unsafe Rust ===");

    let mut num = 42;
    // Creating raw pointers is safe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereferencing raw pointers requires unsafe
    unsafe {
        println!("Raw pointer r1 dereferenced: {}", *r1);
        *r2 = 99;
        println!("Raw pointer r2 modified value: {}", *r2);
    }
    println!("Final variable value: {num}");

    let mut list = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = safe_split_at_mut(&mut list, 3);
    println!("Safe split left: {:?}", left);
    println!("Safe split right: {:?}", right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_split_at_mut() {
        let mut data = vec![10, 20, 30, 40];
        let (first, second) = safe_split_at_mut(&mut data, 2);
        assert_eq!(first, &[10, 20]);
        assert_eq!(second, &[30, 40]);
    }
}
