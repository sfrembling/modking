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

pub fn get_repo() -> anyhow::Result<super::repo::Repo> {
    let repo_path = MKFiles::RepoData.get_path();
    let repo = std::fs::read_to_string(repo_path)?;
    let repo = super::repo::Repo::from_json(&repo)?;

    Ok(repo)
}

pub fn read_directory() -> Vec<PathBuf> {
    let root = MKFiles::MainPath.get_path();
    walkdir::WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter_map(|p| {
            let p = p.strip_prefix(".\\").ok();
            p.map(|p| p.to_path_buf())
        })
        .filter(|p| !p.starts_with(&root))
        .filter(|p| p.is_file())
        .filter(|p| !p.display().to_string().is_empty())
        .collect()
}
