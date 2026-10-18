//! Practice 03: Types and Control Flow
//! Demonstrates scalar & compound types, loops, if let, while let, and match expressions.

fn main() {
    println!("=== 03: Types & Control Flow ===");

    // Compound types: Tuples and Arrays
    let tuple: (i32, f64, &str) = (100, std::f64::consts::PI, "Metrics");
    let (id, score, label) = tuple;
    println!("Tuple unpacked: id={id}, score={score:.4}, label={label}");

    let fixed_array: [i32; 4] = [10, 20, 30, 40];
    println!("Array length: {}, first: {}", fixed_array.len(), fixed_array[0]);

    // Match expression with ranges and guards
    let status_code = 404;
    let message = match status_code {
        200..=299 => "Success",
        400..=499 if status_code == 404 => "Resource Not Found",
        400..=499 => "Client Error",
        500..=599 => "Server Error",
        _ => "Unknown Status",
    };
    println!("HTTP {status_code} => {message}");

    // Loops: loop returning value, while, for in
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop returned value: {result}");

    // If let concise control flow
    let config_opt = Some("ProductionConfig");
    if let Some(config) = config_opt {
        println!("Loaded config: {config}");
    }
}

pub fn classify_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        x if x < 0 => "negative",
        x if x % 2 == 0 => "positive even",
        _ => "positive odd",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(-5), "negative");
        assert_eq!(classify_number(4), "positive even");
        assert_eq!(classify_number(7), "positive odd");
    }
}
