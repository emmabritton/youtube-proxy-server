use rocket::State;
use crate::youtube_manager::YoutubeManager;
use crate::models::channel::Channel;
use rocket_contrib::json::Json;
use anyhow::Result;
use rocket::http::Status;
use crate::models::playlist::Playlist;
use crate::models::video::Video;
use crate::ApiKey;

fn process_single_result<T>(result: Result<Option<T>>) -> Result<Json<T>, Status> {
    if let Err(error) = result {
        eprintln!("{:?}", error);
        return Err(Status::InternalServerError);
    }
    if let Some(item) = result.unwrap() {
        Ok(Json(item))
    } else {
        Err(Status::NotFound)
    }
}

#[get("/v1/channel/<id>")]
pub fn channel(youtube_manager: State<YoutubeManager>, id: String, _api_key: ApiKey) -> Result<Json<Channel>, Status> {
    let channel = youtube_manager.single_channel(id);
    process_single_result(channel)
}

#[get("/v1/video/<id>")]
pub fn video(youtube_manager: State<YoutubeManager>, id: String, _api_key: ApiKey) -> Result<Json<Video>, Status> {
    let video = youtube_manager.single_video(id);
    process_single_result(video)
}

#[get("/v1/playlist/<id>")]
pub fn playlist(youtube_manager: State<YoutubeManager>, id: String, _api_key: ApiKey) -> Result<Json<Playlist>, Status> {
    let playlist = youtube_manager.single_playlist(id);
    process_single_result(playlist)
}