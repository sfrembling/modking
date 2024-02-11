use std::path::PathBuf;

pub enum MKFiles {
    /// The high-level overview of all the repo data
    RepoData,
    /// The location where branch-specified files are stored
    BranchData(String),
    /// The main path for all modking files
    MainPath,
}

impl MKFiles {
    pub fn get_path(&self) -> PathBuf {
        let base = PathBuf::from(".modking");

        match self {
            MKFiles::RepoData => base.join("repo.json"),
            MKFiles::BranchData(branch) => base.join(branch),
            MKFiles::MainPath => base,
        }
    }
}

pub fn init() -> anyhow::Result<()> {
    let main_path = MKFiles::MainPath.get_path();

    if main_path.exists() {
        println!("modking already initialized");
    } else {
        std::fs::create_dir(main_path)?;
        let repo = super::repo::Repo::default().to_json()?;
        let repo_path = MKFiles::RepoData.get_path();
        std::fs::write(repo_path, repo)?;
    }

    Ok(())
}
