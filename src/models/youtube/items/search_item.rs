use anyhow::{Result, Error};
use crate::models::youtube::parts::snippet::Snippet;
use crate::models::youtube::parts::stats::Stats;
use crate::models::youtube::parts::id::Id;
use crate::models::channel::Channel;
use crate::models::video::Video;
use crate::models::playlist::Playlist;
use serde::Deserialize;
use crate::models::youtube::parts::thumbnails::Thumbnails;

#[derive(Deserialize, Debug)]
pub struct SearchItem {
    snippet: Snippet,
    statistics: Option<Stats>,
    id: Id,
}

impl SearchItem {
    pub fn into_channel(self) -> Result<Channel> {
        if self.id.is_channel() {
            Ok(Channel::new(
                self.id.get_id(),
                self.snippet.title,
                self.snippet.thumbnails.unwrap().get_url(),
                None,
                None))
        } else {
            Err(Error::msg("Not a channel"))
        }
    }

    pub fn into_video(self) -> Result<Video> {
        if self.id.is_video() {
            Ok(Video::new(
                self.id.get_id(),
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
        if self.id.is_playlist() {
            Ok(Playlist::new(
                self.id.get_id(),
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