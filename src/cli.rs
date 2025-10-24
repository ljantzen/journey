use clap::{Parser, Subcommand};
use std::path::PathBuf;

fn parse_relative_date(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("Invalid relative date: {}", s))
}

#[derive(Parser)]
#[command(name = "journey")]
#[command(about = "A CLI-based journal application")]
#[command(disable_version_flag = true)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Add a note (default behavior)
    #[arg(short, long)]
    pub add_note: Option<String>,

    /// List notes for a specific date
    #[arg(short, long)]
    pub list: bool,

    /// Edit today's note
    #[arg(short, long)]
    pub edit: bool,

    /// Date selector (YYYY-MM-DD format)
    #[arg(short, long)]
    pub date: Option<String>,

    /// Relative date selector (days offset, 0 = today, positive = past, negative = future)
    #[arg(short, long, value_parser = parse_relative_date)]
    pub relative_date: Option<i64>,

    /// Time selector (HH:MM or HH:MM:SS format)
    #[arg(short, long)]
    pub time: Option<String>,

    /// Time format override (12h|24h)
    #[arg(long)]
    pub time_format: Option<String>,

    /// Read input from stdin (each line becomes a separate note)
    #[arg(long)]
    pub stdin: bool,

    /// Vault name to use (optional if only one vault exists)
    #[arg(short = 'V', long)]
    pub vault: Option<String>,

    /// Show version information
    #[arg(short = 'v', long = "version")]
    pub version: bool,

    /// Note content (for default behavior)
    #[arg(trailing_var_arg = true)]
    pub note_content: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new vault
    Init {
        /// Path to the vault directory
        #[arg(short, long)]
        path: PathBuf,
        
        /// Name of the vault (defaults to path basename if not provided)
        #[arg(short, long)]
        name: Option<String>,
        
        /// Type of vault (table|bullet)
        #[arg(short, long)]
        vault_type: Option<String>,
    },
    /// Add a note
    Add {
        /// The note content
        content: String,
    },
    /// List notes
    List,
    /// Edit notes
    Edit,
}

