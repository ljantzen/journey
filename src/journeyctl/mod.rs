use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "journeyctl")]
#[command(about = "Journey vault management tool")]
pub struct JourneyCtlCli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Show version information
    #[arg(short = 'v', long = "version")]
    pub version: bool,
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

        /// Detect and configure from an existing Obsidian vault
        #[arg(long)]
        obsidian: bool,
    },
    /// List all configured vaults
    List,
    /// Set the default vault
    SetDefault {
        /// Name of the vault to set as default
        vault_name: String,
    },
    /// Unset the default vault
    UnsetDefault,
    /// Show current default vault
    ShowDefault,
    /// Unlist a vault (remove from configuration)
    UnlistVault {
        /// Name of the vault to unlist
        vault_name: String,
    },
    /// Show the location of today's file
    Today {
        /// Name of the vault (uses default if not specified)
        #[arg(short, long)]
        vault: Option<String>,
        
        /// Show detailed information including file existence
        #[arg(long)]
        verbose: bool,
    },
}
