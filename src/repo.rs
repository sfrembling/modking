use std::path::PathBuf;

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
