use clap::Parser;

mod app;
mod filesys;
mod hash;
mod repo;

fn main() {
    let cli = app::Cli::parse();
}
