use std::sync::{Mutex, Arc};
use reqwest::blocking::{Client, Response};
use anyhow::{Error, Result};
use crate::key_manager::KeyManager;
use reqwest::Url;
use std::collections::HashMap;
use crate::models::content_type::ContentType;
use crate::models::youtube::{SearchResponse, ListResponse, PlaylistResponse};
use crate::models::youtube::items::search_item::SearchItem;
use crate::models::youtube::items::list_item::ListItem;
use crate::models::youtube::items::playlist_item::PlaylistItem;
use crate::timer::start_reset_timer;

pub const YOUTUBE_URL: &'static str = "https://www.googleapis.com/youtube/v3";

const COST_SEARCH: usize = 100;
const COST_SINGLE: usize = 6;
const COST_PLAYLIST_PAGE: usize = 3;

pub struct YoutubeClient {
    key_manager: Arc<Mutex<KeyManager>>,
    client: Client,
    base_url: String,
}

impl YoutubeClient {
    pub fn new(key_manager: KeyManager, base_url: String, client: Client) -> YoutubeClient {
        return YoutubeClient {
            key_manager: Arc::new(Mutex::new(key_manager)),
            client,
            base_url,
        };
    }
}

impl YoutubeClient {
    pub fn get_key_status(&self) -> HashMap<usize, usize> {
        self.key_manager.lock().unwrap().get_status()
    }

    pub fn reset_key_status(&self) {
        self.key_manager.lock().unwrap().reset_keys();
    }

    pub fn start_timer(&self) {
        start_reset_timer(self.key_manager.clone())
    }

    pub fn playlist_page(&self, search_params: Vec<(&'static str, String)>) -> Result<(Vec<PlaylistItem>, Option<String>)> {
        let mut params = vec![
            ("part", String::from("id,snippet")),
            ("maxResults", String::from("50"))];

        for (key, value) in &search_params {
            params.push((key, value.clone()));
        }

        self.request(COST_PLAYLIST_PAGE, "playlist items", params, "playlistItems", |resp| {
            let response = resp.json::<PlaylistResponse>()?;
            Ok(Some((response.items, response.next_page_token)))
        }).map(|result| result.unwrap())
    }

    pub fn single(&self, content_type: ContentType, id: String) -> Result<Option<ListItem>> {
        let mut params: Vec<(&str, String)> = vec![("id", id)];

        let path;

        match content_type {
            ContentType::CHANNEL => {
                path = "channels";
                params.push(("part", String::from("snippet,id,statistics,contentDetails")));
            }
            ContentType::VIDEO => {
                path = "videos";
                params.push(("part", String::from("snippet,id")));
            }
            ContentType::PLAYLIST => {
                path = "playlists";
                params.push(("part", String::from("snippet,id")));
            }
        }

        self.request(COST_SINGLE, "single", params, path, |resp| {
            let response = resp.json::<ListResponse>()?;
            if response.items.is_some() {
                let mut list = response.items.unwrap();
                if list.len() > 0 {
                    Ok(Some(list.remove(0)))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        })
    }

    pub fn search(&self, content_type: ContentType, search_params: Vec<(&'static str, String)>) -> Result<Vec<SearchItem>> {
        let mut params = vec![
            ("part", String::from("snippet")),
            ("maxResults", String::from("50")),
            ("safeSearch", String::from("none")),
            ("order", String::from("date"))];

        for (key, value) in &search_params {
            params.push((key, value.clone()));
        }

        match content_type {
            ContentType::CHANNEL => params.push(("type", String::from("channel"))),
            ContentType::VIDEO => {
                params.push(("type", String::from("video")));
                params.push(("videoDimension", String::from("2d")))
            }
            ContentType::PLAYLIST => params.push(("type", String::from("playlist"))),
        }

        self.request(COST_SEARCH, "search", params, "search", |resp| {
            Ok(Some(resp.json::<SearchResponse>()?.items))
        }).map(|result| result.unwrap())
    }

    pub fn request<T, F: Fn(Response) -> Result<Option<T>>>(&self, cost: usize, key_error_name: &'static str, mut params: Vec<(&'static str, String)>, path: &str, response_handler: F) -> Result<Option<T>> {
        let key = self.key_manager.lock().unwrap().get_key(cost);

        if let None = key {
            return Err(Error::msg(format!("No keys available for {}", key_error_name)));
        }

        let key = key.unwrap();

        params.push(("key", key.clone()));

        let resp = self.client
            .get(Url::parse_with_params(&format!("{}/{}", self.base_url, path), &params)?)
            .send();

        match resp {
            Ok(resp) => {
                if resp.status().is_success() {
                    response_handler(resp)
                } else if resp.status().as_u16() == 429 {
                    self.key_manager.lock().unwrap().set_key_as_expired(&key);
                    self.request(cost, key_error_name, params, path, response_handler)
                } else {
                    let status_code = resp.status().as_u16();
                    eprintln!("{} Error: {}\n{}", key_error_name, status_code, resp.text().unwrap_or(String::from("Unable to parse response")));
                    Err(Error::msg(format!("Error getting {}: {}", key_error_name, status_code)))
                }
            }
            Err(err) => Err(Error::from(err))
        }
    }
}