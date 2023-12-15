use chrono::{DateTime, Utc};
use cid::Cid;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use crate::error::{KrondorError, KrondorResult};

#[cfg(not(target_arch = "wasm32"))]
const SHA3_256_CODEC: u64 = 0x16;
#[cfg(not(target_arch = "wasm32"))]
const RAW_CODEC: u64 = 0x55;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Item {
    name: String,
    date: DateTime<Utc>,
    title: String,
    description: String,
    cid: Cid,
    render: Render,
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }
    #[cfg(target_arch = "wasm32")]
    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
    #[cfg(target_arch = "wasm32")]
    pub fn title(&self) -> &str {
        &self.title
    }
    #[cfg(target_arch = "wasm32")]
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn cid(&self) -> &Cid {
        &self.cid
    }
    pub fn render(&self) -> &Render {
        &self.render
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Render {
    Markdown,
    Html,
    Jpg,
    Mp3,
    Blank,
}

impl Default for Render {
    fn default() -> Self {
        Self::Blank
    }
}

impl std::fmt::Display for Render {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Markdown => "Markdown",
            Self::Html => "Html",
            Self::Jpg => "Jpg",
            Self::Mp3 => "Mp3",
            Self::Blank => "Blank",
        };
        write!(f, "{}", s)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::path::PathBuf> for Render {
    fn from(path: std::path::PathBuf) -> Self {
        let ext = path.extension().unwrap().to_str().unwrap();
        match ext {
            "md" => Self::Markdown,
            "html" => Self::Html,
            "jpg" => Self::Jpg,
            "mp3" => Self::Mp3,
            _ => Self::Blank,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Item {
    pub fn from_disk(name: &str, title: &str, description: &str) -> KrondorResult<Self> {
        use crate::utils::fs::post_path;
        use multihash::Multihash;
        use sha3::{Digest, Sha3_256};

        let path = post_path(name);
        let mut file = std::fs::File::open(&path).map_err(KrondorError::Io)?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut bytes).map_err(KrondorError::Io)?;
        let hash = Sha3_256::digest(&bytes);
        let mh = Multihash::wrap(SHA3_256_CODEC, &hash).map_err(KrondorError::Multihash)?;
        let cid = Cid::new_v1(RAW_CODEC, mh);
        Ok(Self {
            name: name.to_string(),
            date: DateTime::from(std::fs::metadata(&path).unwrap().modified().unwrap()),
            title: title.to_string(),
            description: description.to_string(),
            cid,
            render: path.into(),
        })
    }
}
