use std::{fs, io::ErrorKind, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub user_id: String,
    pub api_key: String,
}

impl Credentials {
    pub fn new(user_id: String, api_key: String) -> Self {
        Self { user_id, api_key }
    }

    pub fn read_from_file(path: PathBuf) -> eyre::Result<Self> {
        let file_content = fs::read_to_string(path);

        match file_content {
            Err(err) if err.kind() == ErrorKind::NotFound => {
                Err(eyre::eyre!("No credentials found. Please log in first"))
            }
            Err(err) => Err(err.into()),
            Ok(file_content) => {
                let credentials: Self = toml::from_str(&file_content)?;

                Ok(credentials)
            }
        }
    }

    pub fn save_to_file(&self, path: PathBuf) -> eyre::Result<()> {
        let file_content = toml::to_string(self)?;

        // make sure that the directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, file_content)?;

        Ok(())
    }
}

fn get_credentials() -> eyre::Result<Credentials> {
    Ok(Credentials {
        user_id: "12147112".into(),
        api_key: "s0YxVw53TiNR208cbbgI8Klz".into(),
    })
}
