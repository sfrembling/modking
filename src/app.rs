use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Init this dir as a modking repo
    Init {
        /// Run through all of the init steps automatically
        #[arg(short, long)]
        full: bool,
    },
    /// Lock the current branch
    Lock,
    /// Unlock the current branch
    Unlock,
    /// Set the current branch to the state of this directory
    Update,
    /// Swap to a new or existing branch
    Branch {
        /// Interactively select a branch
        #[arg(short, long)]
        list: bool,
        /// Create a new branch
        #[arg(short, long)]
        new: Option<String>,
        /// Return to the vanilla branch
        #[arg(short, long)]
        vanilla: bool,
    },
}
