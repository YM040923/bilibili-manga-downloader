use serde::{Deserialize, Serialize};

pub type ImageTokenRespData = Vec<ImageTokenItemRespData>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageTokenItemRespData {
    #[serde(default, alias = "complete_url")]
    pub complete_url: String,
    #[serde(default, alias = "hit_encrypt")]
    pub hit_encrpyt: bool,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub token: String,
}
