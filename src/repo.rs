use std::{fmt::Display, path::PathBuf, sync::Mutex};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Repo {
    branches: Vec<Branch>,
    current_branch: usize,
}

impl Repo {
    pub fn to_json(&self) -> anyhow::Result<String> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }

    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        let repo: Repo = serde_json::from_str(json)?;
        Ok(repo)
    }

    pub fn get_current_branch_mut(&mut self) -> &mut Branch {
        &mut self.branches[self.current_branch]
    }

    pub fn get_current_branch(&self) -> &Branch {
        &self.branches[self.current_branch]
    }
}

impl Default for Repo {
    fn default() -> Self {
        Self {
            branches: vec![Branch::default()],
            current_branch: 0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Branch {
    name: String,
    refs: Vec<Ref>,
    locked: bool,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.locked { "🔒" } else { "🔓" };
        write!(f, "{} - {}", self.name, status)
    }
}

impl Branch {
    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        if self.locked {
            return Err(anyhow::anyhow!(
                "{} is locked; use `unlock` to unlock it",
                self.name
            ));
        }

        let files = super::filesys::read_directory();
        let this = Mutex::new(vec![]);
        let pb = super::interact::progress_bar_with_length("Hashing files", files.len() as u64);

        files.par_iter().for_each(|path| {
            let mut contents = this.lock().unwrap();
            let hash = super::hash::hash(path).unwrap();
            let ref_ = Ref {
                path: path.clone(),
                hash,
            };
            contents.push(ref_);
            pb.inc(1);
        });

        pb.finish();

        let refs = this.into_inner().unwrap();

        self.refs = refs;
        Ok(())
    }
}

impl Default for Branch {
    fn default() -> Self {
        Self {
            name: "vanilla".to_string(),
            refs: Vec::new(),
            locked: false,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ref {
    path: PathBuf,
    hash: String,
}
