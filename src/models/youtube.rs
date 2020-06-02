use serde::Deserialize;
use crate::models::channel::Channel;
use crate::models::video::Video;
use crate::models::playlist::Playlist;
use anyhow::{Result, Error};

impl SearchResult {
    pub fn into_channel(self) -> Result<Channel> {
        if self.is_channel() {
            let stats = self.statistics.unwrap_or(Stats { video_count: 0 });
            Ok(Channel::new(
                self.id.channel_id.unwrap(),
                self.snippet.title,
                self.snippet.thumbnails.unwrap().get_url(),
                stats.video_count))
        } else {
            Err(Error::msg("Not a channel"))
        }
    }

    pub fn into_video(self) -> Result<Video> {
        if self.is_video() {
            Ok(Video::new(
                self.id.video_id.unwrap(),
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
        if self.is_playlist() {
            Ok(Playlist::new(
                self.id.playlist_id.unwrap(),
                self.snippet.title,
                self.snippet.thumbnails.unwrap_or(Thumbnails::empty()).get_url(),
                self.snippet.channel_id.unwrap(),
                self.snippet.channel_title.unwrap(),
            ))
        } else {
            Err(Error::msg("Not a playlist"))
        }
    }

    fn is_channel(&self) -> bool {
        self.id.kind == "youtube#channel"
    }

    fn is_video(&self) -> bool {
        self.id.kind == "youtube#video"
    }

    fn is_playlist(&self) -> bool {
        self.id.kind == "youtube#playlist"
    }
}

////////
//List types
////////

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub items: Vec<SearchResult>
}

////////
//Item types
////////

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    snippet: Snippet,
    statistics: Option<Stats>,
    id: Id,
}

#[derive(Deserialize, Debug)]
pub struct ListItem {
    kind: String,
    snippet: Snippet,
    statistics: Option<Stats>,
    id: String,
}

////////
//Common
////////

impl Thumbnails {
    fn empty() -> Thumbnails {
        Thumbnails {
            default: { Thumbnail { url: String::new() } },
            medium: None,
            high: None,
        }
    }

    fn get_url(&self) -> String {
        (if let Some(high) = &self.high {
            &high.url
        } else if let Some(med) = &self.medium {
            &med.url
        } else {
            &self.default.url
        }).clone()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    kind: String,
    channel_id: Option<String>,
    playlist_id: Option<String>,
    video_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    title: String,
    channel_id: Option<String>,
    channel_title: Option<String>,
    description: Option<String>,
    published_at: Option<String>,
    thumbnails: Option<Thumbnails>,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnails {
    high: Option<Thumbnail>,
    medium: Option<Thumbnail>,
    default: Thumbnail,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnail {
    url: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    video_count: u64
}