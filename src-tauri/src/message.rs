// add these use statements to the top
use anyhow::Result;
use iroh::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message {
    AboutMe { from: NodeId, name: String },
    Message { from: NodeId, text: String },
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}
