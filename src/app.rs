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
    Info,
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        let mut repo = super::filesys::get_repo()?;
        match self.cmd {
            Command::Init { full } => {
                super::filesys::init()?;
                if full {
                    repo.get_current_branch_mut().update()?;
                    repo.get_current_branch_mut().lock();
                    std::fs::write(
                        super::filesys::MKFiles::RepoData.get_path(),
                        repo.to_json()?,
                    )?;
                }
            }
            Command::Lock => {
                repo.get_current_branch_mut().lock();
                std::fs::write(
                    super::filesys::MKFiles::RepoData.get_path(),
                    repo.to_json()?,
                )?;
            }
            Command::Unlock => {
                repo.get_current_branch_mut().unlock();
                std::fs::write(
                    super::filesys::MKFiles::RepoData.get_path(),
                    repo.to_json()?,
                )?;
            }
            Command::Update => {
                repo.get_current_branch_mut().update()?;
                std::fs::write(
                    super::filesys::MKFiles::RepoData.get_path(),
                    repo.to_json()?,
                )?;
            }
            Command::Branch { list, new, vanilla } => {
                if list {
                    let branches = repo.get_all_branches();
                    let choice = dialoguer::Select::new()
                        .items(&branches)
                        .default(repo.current_branch)
                        .interact()?;

                    repo.swap_current(choice);
                    std::fs::write(
                        super::filesys::MKFiles::RepoData.get_path(),
                        repo.to_json()?,
                    )?;
                } else if vanilla {
                    repo.swap_current(0);
                    std::fs::write(
                        super::filesys::MKFiles::RepoData.get_path(),
                        repo.to_json()?,
                    )?;
                } else if let Some(name) = new {
                    repo.duplicate(&name)?;
                    std::fs::write(
                        super::filesys::MKFiles::RepoData.get_path(),
                        repo.to_json()?,
                    )?;
                }
            }
            Command::Info => println!("{}", repo.get_current_branch()),
        }

        Ok(())
    }
}
