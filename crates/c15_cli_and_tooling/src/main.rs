//! Practice 15: CLI & Tooling with Clap and Serde
//! Demonstrates declarative CLI parsing, subcommands, and JSON formatting.

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "cloud-cli", version = "1.0.0", about = "CLI Tool for Cloud Practice")]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Inspect a given cloud resource
    Inspect {
        #[arg(short, long)]
        resource_id: String,
    },
    /// Format data as JSON
    Format {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ServiceRecord {
    pub name: String,
    pub port: u16,
}

pub fn format_record(name: &str, port: u16) -> String {
    let record = ServiceRecord {
        name: name.to_string(),
        port,
    };
    serde_json::to_string_pretty(&record).unwrap()
}

fn main() {
    println!("=== 15: CLI & Tooling ===");
    let args = Cli::parse_from(["cloud-cli", "format", "--name", "api-gateway", "--port", "8443"]);

    match args.command {
        Commands::Inspect { resource_id } => {
            println!("Inspecting resource: {resource_id}");
        }
        Commands::Format { name, port } => {
            let json = format_record(&name, port);
            println!("Generated JSON configuration:
{json}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_record() {
        let json = format_record("auth-service", 9000);
        let parsed: ServiceRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed,
            ServiceRecord {
                name: "auth-service".to_string(),
                port: 9000
            }
        );
    }
}
