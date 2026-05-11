use serde::{Deserialize, Serialize};
use specta::Type;

/// B站漫画搜索响应（2026年新版扁平结构）
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchRespData {
    pub list: Vec<ComicInSearchRespData>,
    #[serde(rename = "total_page", default)]
    pub total_page: i64,
    #[serde(rename = "total_num", default)]
    pub total_num: i64,
    #[serde(default)]
    pub similar: String,
    #[serde(rename = "se_id", default)]
    pub se_id: String,
    #[serde(default)]
    pub banner: Option<BannerRespData>,
    #[serde(skip, default)]
    pub jump: Option<serde_json::Value>,
    #[serde(skip, default)]
    pub recommends: Vec<serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ComicInSearchRespData {
    pub id: i64,
    pub title: String,
    #[serde(rename = "square_cover")]
    pub square_cover: String,
    #[serde(rename = "vertical_cover")]
    pub vertical_cover: String,
    #[serde(rename = "author_name")]
    pub author_name: Vec<String>,
    pub styles: Vec<String>,
    #[serde(rename = "is_finish")]
    pub is_finish: i64,
    #[serde(rename = "allow_wait_free")]
    pub allow_wait_free: bool,
    #[serde(rename = "discount_type")]
    pub discount_type: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub wiki: WikiRespData,
    #[serde(skip, default)]
    pub alia_title: Vec<String>,
    #[serde(default, rename = "horizontal_cover")]
    pub horizontal_cover: String,
    #[serde(default, rename = "jump_value")]
    pub jump_value: String,
    #[serde(default)]
    pub attribution: i64,
    #[serde(default)]
    pub numbers: i64,
    #[serde(default, rename = "org_title")]
    pub org_title: String,
    #[serde(default, rename = "real_title")]
    pub real_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiRespData {
    pub id: i64,
    pub title: String,
    #[serde(rename = "origin_title")]
    pub origin_title: String,
    #[serde(rename = "vertical_cover")]
    pub vertical_cover: String,
    pub producer: String,
    #[serde(rename = "author_name")]
    pub author_name: Vec<String>,
    #[serde(rename = "publish_time")]
    pub publish_time: String,
    pub frequency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct BannerRespData {
    pub icon: String,
    pub title: String,
    pub url: String,
}
