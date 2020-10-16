use crate::models::youtube::parts::snippet::Snippet;
use crate::models::youtube::parts::stats::Stats;
use crate::models::youtube::parts::content_details::ContentDetails;
use crate::models::channel::Channel;
use crate::models::video::Video;
use anyhow::{Result, Error};
use serde::Deserialize;
use crate::models::playlist::Playlist;
use crate::models::youtube::parts::thumbnails::Thumbnails;
use crate::models::youtube::parts::kind::Kind;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    kind: Kind,
    snippet: Snippet,
    statistics: Option<Stats>,
    id: String,
    content_details: Option<ContentDetails>,
}

impl ListItem {
    pub fn into_channel(self) -> Result<Channel> {
        if self.kind.is_channel() {
            let stats = self.statistics.unwrap_or(Stats::default());
            Ok(Channel::new(
                self.id,
                self.snippet.title,
                self.snippet.thumbnails.unwrap().get_url(),
                Some(stats.get_video_count()),
                self.content_details.and_then(|unwrapped| unwrapped.get_upload_playlist_id())))
        } else {
            Err(Error::msg("Not a channel"))
        }
    }

    pub fn into_video(self) -> Result<Video> {
        if self.kind.is_video() {
            Ok(Video::new(
                self.id,
                self.snippet.title,
                self.snippet.published_at.unwrap(),
                self.snippet.thumbnails.unwrap().get_url(),
                self.snippet.channel_id.unwrap(),
                self.snippet.channel_title.unwrap(),
                self.snippet.description,
            ))
        } else {
            Err(Error::msg("Not a video"))
        }
    }

    pub fn into_playlist(self) -> Result<Playlist> {
        if self.kind.is_playlist() {
            Ok(Playlist::new(
                self.id,
                self.snippet.title,
                self.snippet.thumbnails.unwrap_or(Thumbnails::empty()).get_url(),
                self.snippet.channel_id.unwrap(),
                self.snippet.channel_title.unwrap(),
            ))
        } else {
            Err(Error::msg("Not a playlist"))
        }
    }
}