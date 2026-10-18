//! Practice 01: Getting Started
//! Demonstrates variables, mutability, shadowing, constants, and basic output.

const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("=== 01: Getting Started ===");
    println!("Max points constant: {MAX_POINTS}");

    // Immutability by default
    let x = 5;
    println!("x = {x}");

    // Mutability with `mut`
    let mut y = 10;
    println!("y initial = {y}");
    y += 5;
    println!("y after mutation = {y}");

    // Shadowing (re-declaring with let, allows type change)
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces count through shadowing: {spaces}");

    let result = add(x, y);
    println!("add({x}, {y}) = {result}");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }
}
