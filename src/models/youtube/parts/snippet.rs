use serde::Deserialize;
use crate::models::youtube::parts::thumbnails::Thumbnails;
use crate::models::youtube::parts::resource_id::ResourceId;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub title: String,
    pub channel_id: Option<String>,
    pub channel_title: Option<String>,
    pub description: Option<String>,
    pub published_at: Option<String>,
    pub thumbnails: Option<Thumbnails>,
    pub resource_id: Option<ResourceId>
}
