use serde::Deserialize;

use crate::credentials::Credentials;

static ZOTERO_BASE_URL: &str = "https://api.zotero.org";

pub struct ZoteroClient {
    cred: Credentials,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Deserialize)]
pub struct Collection {
    pub key: String,
    pub data: CollectionData,
}

#[derive(Debug, Deserialize)]
pub struct CollectionData {
    pub name: String,
}

impl ZoteroClient {
    pub fn new(cred: Credentials) -> Self {
        Self {
            cred,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_collections(&self) -> eyre::Result<Vec<Collection>> {
        let res: Vec<Collection> = self
            .client
            .get(format!(
                "{}/users/{}/collections",
                ZOTERO_BASE_URL, self.cred.user_id
            ))
            .header("Authorization", format!("Bearer {}", self.cred.api_key))
            .send()?
            .json()?;

        Ok(res)
    }

    pub fn get_bibliography(&self, collection_key: String) -> reqwest::Result<bytes::Bytes> {
        self.client
            .get(format!(
                "{}/users/{}/collections/{}/items",
                ZOTERO_BASE_URL, self.cred.user_id, collection_key
            ))
            .query(&[("format", "biblatex")])
            .header("Authorization", format!("Bearer {}", self.cred.api_key))
            .send()?
            .bytes()
    }
}
