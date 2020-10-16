use crate::youtube_manager::YoutubeManager;
use crate::ApiKey;
use rocket_contrib::json::Json;
use crate::models::video::Video;
use rocket::State;
use anyhow::Result;
use rocket::http::Status;

#[get("/v1/channel/<id>/most_recent")]
pub fn get_most_recent_videos_for_channel(youtube_manager: State<YoutubeManager>, id: String, _api_key: ApiKey) -> Result<Json<Vec<Video>>> {
    youtube_manager.list_latest_videos_for_channel(id)
        .map(|list| Json(list))
}

#[get("/v1/channel/<id>/videos")]
pub fn get_videos_for_channel(youtube_manager: State<YoutubeManager>, id: String, _api_key: ApiKey) -> Result<Json<Vec<Video>>> {
    let channel_result = youtube_manager.single_channel(id)?;

    match channel_result {
        None => {
            Err(anyhow::Error::msg(Status::NotFound.reason))
        },
        Some(channel) => {
            let playlist_id = channel.get_all_videos_playlist_id().unwrap();
            get_videos_for_playlist(youtube_manager, playlist_id, _api_key)
        }
    }
}

#[get("/v1/playlist/<id>/videos")]
pub fn get_videos_for_playlist(youtube_manager: State<YoutubeManager>, id: String, _api_key: ApiKey) -> Result<Json<Vec<Video>>> {
    let mut page_token: Option<String> = None;
    let mut results: Vec<Video> = vec![];

    loop {
        let (videos, next_page_token) = youtube_manager.list_videos_for_playlist(id.clone(), page_token.clone())?;
        for video in videos {
            &results.push(video);
        }
        page_token = next_page_token;
        if page_token.is_none() {
            break;
        }
    }

    Ok(Json(results))
}
