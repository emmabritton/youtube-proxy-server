use crate::key_manager::KeyManager;
use std::time::Duration;
use anyhow::{Error, Result};
use reqwest::blocking::{Client};
use std::sync::Mutex;
use reqwest::Url;
use reqwest::Proxy;
use std::collections::HashMap;
use crate::models::content_type::ContentType;
use crate::models::youtube::{SearchResult, SearchResponse};
use crate::models::channel::Channel;
use crate::models::playlist::Playlist;
use crate::models::video::Video;

const YOUTUBE_URL: &'static str = "https://www.googleapis.com/youtube/v3";
const TIMEOUT: u64 = 120;

const COST_SEARCH: usize = 100;

pub struct YoutubeManager {
    key_manager: Mutex<KeyManager>,
    client: Client,
}

impl YoutubeManager {
    pub fn new(key_manager: KeyManager) -> YoutubeManager {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(TIMEOUT))
            .timeout(Duration::from_secs(TIMEOUT))
            .build()
            .unwrap();

        return YoutubeManager {
            key_manager: Mutex::new(key_manager),
            client
        };
    }
}

impl YoutubeManager {
    pub fn get_key_status(&self) -> HashMap<usize, usize> {
        self.key_manager.lock().unwrap().get_status()
    }

    pub fn single_video(&self, video_id: String) {}

    pub fn single_channel(&self, channel_id: String) {}

    pub fn single_playlist(&self, playlist_id: String) {}

    pub fn search_channel(&self, search_query: String) -> Result<Vec<Channel>> {
        let channels = self.search(ContentType::CHANNEL, search_query)?
            .into_iter()
            .map(|item| item.into_channel().unwrap())
            .collect();
        Ok(channels)
    }

    pub fn search_video(&self, search_query: String) -> Result<Vec<Video>> {
        let channels = self.search(ContentType::VIDEO, search_query)?
            .into_iter()
            .map(|item| item.into_video().unwrap())
            .collect();
        Ok(channels)
    }

    pub fn search_playlist(&self, search_query: String) -> Result<Vec<Playlist>> {
        let channels = self.search(ContentType::PLAYLIST, search_query)?
            .into_iter()
            .map(|item| item.into_playlist().unwrap())
            .collect();
        Ok(channels)
    }

    fn search(&self, content_type: ContentType, search_query: String) -> Result<Vec<SearchResult>> {
        let key = self.key_manager.lock().unwrap().get_key(COST_SEARCH);

        if let None = key {
            return Err(Error::msg("No keys available for search"));
        }

        let key = key.unwrap();

        let mut params = vec![
            ("part", "snippet"),
            ("key", &key),
            ("maxResults", "50"),
            ("safeSearch", "none"),
            ("order", "date"),
            ("q", &search_query)];

        match content_type {
            ContentType::CHANNEL => params.push(("type", "channel")),
            ContentType::VIDEO => {
                params.push(("type", "video"));
                params.push(("videoDimension", "2d"))
            },
            ContentType::PLAYLIST => params.push(("type", "playlist")),
        }

        let resp = self.client
            .get(Url::parse_with_params(&format!("{}/search", YOUTUBE_URL), &params)?)
            .send();

        match resp {
            Ok(resp) => {
                if resp.status().is_success()  {
                    let items = resp.json::<SearchResponse>()?
                        .items;
                    Ok(items)
                } else if resp.status().as_u16() == 429  {
                    self.key_manager.lock().unwrap().set_key_as_expired(key);
                    self.search(content_type, search_query)
                } else {
                    Err(Error::msg(format!("Error searching: {}", resp.status().as_u16())))
                }
            }
            Err(err) => Err(Error::from(err))
        }
    }
}