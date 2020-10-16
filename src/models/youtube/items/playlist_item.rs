use crate::models::youtube::parts::snippet::Snippet;
use crate::models::video::Video;
use anyhow::{Result, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    snippet: Snippet,
    id: String
}

impl PlaylistItem {
    pub fn into_video(self) -> Result<Video> {
        let resource_id = self.snippet.resource_id.unwrap();
        if resource_id.kind.is_video() {
            Ok(Video::new(
                resource_id.video_id,
                self.snippet.title,
                self.snippet.published_at.unwrap(),
                self.snippet.thumbnails.unwrap().get_url(),
                self.snippet.channel_id.unwrap(),
                self.snippet.channel_title.unwrap(),
                self.snippet.description,
            ))
        } else {
            Err(Error::msg("Not a video (in playlist)"))
        }
    }
}