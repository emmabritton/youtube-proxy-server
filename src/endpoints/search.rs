use rocket::State;
use crate::youtube_manager::YoutubeManager;
use crate::models::channel::Channel;
use rocket_contrib::json::Json;
use anyhow::Result;
use crate::models::playlist::Playlist;
use crate::models::video::Video;
use crate::ApiKey;

#[get("/v1/search/channel?<q>")]
pub fn channel(youtube_manager: State<YoutubeManager>, q: String, _api_key: ApiKey) -> Result<Json<Vec<Channel>>> {
    youtube_manager.search_channel(q)
        .map(|list| Json(list))
}

#[get("/v1/search/video?<q>")]
pub fn video(youtube_manager: State<YoutubeManager>, q: String, _api_key: ApiKey) -> Result<Json<Vec<Video>>> {
    youtube_manager.search_video( q)
        .map(|list| Json(list))
}

#[get("/v1/search/playlist?<q>")]
pub fn playlist(youtube_manager: State<YoutubeManager>, q: String, _api_key: ApiKey) -> Result<Json<Vec<Playlist>>> {
    youtube_manager.search_playlist(q)
        .map(|list| Json(list))
}