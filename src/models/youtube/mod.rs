use serde::Deserialize;
use crate::models::youtube::items::list_item::ListItem;
use crate::models::youtube::items::search_item::SearchItem;
use crate::models::youtube::items::playlist_item::PlaylistItem;

pub mod parts;
pub mod items;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub items: Vec<SearchItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub items: Option<Vec<ListItem>>,
    pub page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub total_results: Option<usize>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResponse {
    pub items: Vec<PlaylistItem>,
    pub next_page_token: Option<String>,
}
