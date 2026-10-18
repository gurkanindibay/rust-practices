//! Practice 06: Error Handling
//! Demonstrates Result<T, E>, the `?` operator, custom errors with `thiserror`, and application errors with `anyhow`.

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("Config file not found at: {path}")]
    NotFound { path: String },
    #[error("Failed to parse port number: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
    #[error("Validation failed: {0}")]
    Validation(String),
}

pub fn parse_port_entry(entry: &str) -> Result<u16, ConfigError> {
    let port: u16 = entry.trim().parse()?;
    if port < 1024 {
        return Err(ConfigError::Validation(format!(
            "Port {port} is reserved for privileged services"
        )));
    }
    Ok(port)
}

pub fn load_service_config(port_str: &str) -> anyhow::Result<u16> {
    let port = parse_port_entry(port_str)?;
    Ok(port)
}

fn main() -> anyhow::Result<()> {
    println!("=== 06: Error Handling ===");

    match parse_port_entry("8080") {
        Ok(port) => println!("Valid port parsed: {port}"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match parse_port_entry("80") {
        Ok(port) => println!("Port: {port}"),
        Err(e) => println!("Expected validation error caught: {e}"),
    }

    let port = load_service_config("9090")?;
    println!("Config successfully loaded with port: {port}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_port() {
        assert_eq!(parse_port_entry("3000").unwrap(), 3000);
    }

    #[test]
    fn test_privileged_port() {
        match parse_port_entry("80") {
            Err(ConfigError::Validation(_)) => (),
            _ => panic!("Expected Validation error for port 80"),
        }
    }

    #[test]
    fn test_invalid_syntax() {
        match parse_port_entry("not_a_number") {
            Err(ConfigError::InvalidPort(_)) => (),
            _ => panic!("Expected InvalidPort error"),
        }
    }
}
