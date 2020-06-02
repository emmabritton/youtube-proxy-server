use rocket::State;
use crate::youtube_manager::YoutubeManager;
use crate::models::content_type::ContentType;
use crate::models::channel::Channel;
use rocket_contrib::json::Json;
use anyhow::Result;
use crate::models::playlist::Playlist;
use crate::models::video::Video;

#[get("/v1/youtube/search/channel?<q>")]
pub fn channel(youtube_manager: State<YoutubeManager>, q: String) -> Result<Json<Vec<Channel>>> {
    youtube_manager.search_channel(q)
        .map(|list| Json(list))
}

#[get("/v1/youtube/search/video?<q>")]
pub fn video(youtube_manager: State<YoutubeManager>, q: String) -> Result<Json<Vec<Video>>> {
    youtube_manager.search_video( q)
        .map(|list| Json(list))
}

#[get("/v1/youtube/search/playlist?<q>")]
pub fn playlist(youtube_manager: State<YoutubeManager>, q: String) -> Result<Json<Vec<Playlist>>> {
    youtube_manager.search_playlist(q)
        .map(|list| Json(list))
}