use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchRespData {
    #[serde(rename = "comic_data")]
    pub comic_data: SearchComicRespData,
    #[serde(rename = "novel_data")]
    pub novel_data: SearchNovelRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchComicRespData {
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
    #[serde(default, rename = "has_complete_match")]
    pub has_complete_match: bool,
    #[serde(skip, default)]
    pub intervene: Option<serde_json::Value>,
    #[serde(skip, default)]
    pub jump: Option<serde_json::Value>,
    #[serde(default, rename = "no_result_reason")]
    pub no_result_reason: i64,
    #[serde(skip, default)]
    pub recommends: Vec<serde_json::Value>,
    #[serde(default, rename = "search_no_result_recommend_text")]
    pub search_no_result_recommend_text: String,
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
    pub goods: Option<serde_json::Value>,
    #[serde(default)]
    pub recommend: String,
    #[serde(skip, default)]
    pub review: Option<serde_json::Value>,
    #[serde(default, rename = "short_intro")]
    pub short_intro: String,
    #[serde(skip, default)]
    pub tags: Vec<serde_json::Value>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchNovelRespData {
    pub total: i64,
    pub list: Vec<NovelInSearchRespData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct NovelInSearchRespData {
    #[serde(rename = "novel_id")]
    pub novel_id: i64,
    pub title: String,
    #[serde(rename = "v_cover")]
    pub v_cover: String,
    #[serde(rename = "finish_status")]
    pub finish_status: i64,
    pub status: i64,
    #[serde(rename = "discount_type")]
    pub discount_type: i64,
    pub numbers: i64,
    pub style: StyleRespData,
    pub evaluate: String,
    pub author: String,
    pub tag: TagRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct StyleRespData {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TagRespData {
    pub id: i64,
    pub name: String,
}
