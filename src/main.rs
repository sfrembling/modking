use clap::Parser;

mod app;
mod filesys;
mod hash;
mod interact;
mod repo;

fn main() -> anyhow::Result<()> {
    let cli = app::Cli::parse();

    cli.run()?;

    Ok(())
}
