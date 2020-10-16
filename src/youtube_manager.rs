use crate::key_manager::KeyManager;
use std::time::Duration;
use anyhow::Result;
use reqwest::blocking::{Client};
use std::collections::HashMap;
use crate::models::content_type::ContentType;
use crate::models::channel::Channel;
use crate::models::playlist::Playlist;
use crate::models::video::Video;
use crate::youtube_client::YoutubeClient;

const TIMEOUT: u64 = 120;

pub struct YoutubeManager {
    client: YoutubeClient
}

impl YoutubeManager {
    pub fn new(key_manager: KeyManager, base_url: String, proxy: &Option<String>) -> YoutubeManager {
        let mut builder = Client::builder()
            .connect_timeout(Duration::from_secs(TIMEOUT))
            .timeout(Duration::from_secs(TIMEOUT));

        if let Some(proxy) = proxy {
            builder = builder.proxy(reqwest::Proxy::all(proxy).unwrap())
        }

        let client = builder.build().unwrap();

        let youtube_client = YoutubeClient::new(key_manager, base_url, client);

        return YoutubeManager {
            client: youtube_client
        };
    }
}

impl YoutubeManager {
    pub fn get_key_status(&self) -> HashMap<usize, usize> {
        self.client.get_key_status()
    }

    pub fn single_video(&self, video_id: String) -> Result<Option<Video>> {
        let result = self.client.single(ContentType::VIDEO, video_id)?;
        let video = result.map(|item| item.into_video().unwrap());
        Ok(video)
    }

    pub fn single_channel(&self, channel_id: String) -> Result<Option<Channel>> {
        let result = self.client.single(ContentType::CHANNEL, channel_id)?;
        let channel = result.map(|item| item.into_channel().unwrap());
        Ok(channel)
    }

    pub fn single_playlist(&self, playlist_id: String) -> Result<Option<Playlist>> {
        let result = self.client.single(ContentType::PLAYLIST, playlist_id)?;
        let playlist = result.map(|item| item.into_playlist().unwrap());
        Ok(playlist)
    }

    pub fn search_channel(&self, search_query: String) -> Result<Vec<Channel>> {
        let search_params = vec![("q", search_query)];
        let channels = self.client.search(ContentType::CHANNEL, search_params)?
            .into_iter()
            .map(|item| item.into_channel().unwrap())
            .collect();
        Ok(channels)
    }

    pub fn search_video(&self, search_query: String) -> Result<Vec<Video>> {
        let search_params = vec![("q", search_query)];
        let channels = self.client.search(ContentType::VIDEO, search_params)?
            .into_iter()
            .map(|item| item.into_video().unwrap())
            .collect();
        Ok(channels)
    }

    pub fn search_playlist(&self, search_query: String) -> Result<Vec<Playlist>> {
        let search_params = vec![("q", search_query)];
        let channels = self.client.search(ContentType::PLAYLIST, search_params)?
            .into_iter()
            .map(|item| item.into_playlist().unwrap())
            .collect();
        Ok(channels)
    }

    pub fn list_latest_videos_for_channel(&self, id: String) -> Result<Vec<Video>> {
        let search_params = vec![("channelId", id)];
        let videos = self.client.search(ContentType::VIDEO, search_params)?
            .into_iter()
            .map(|item| item.into_video().unwrap())
            .collect();
        Ok(videos)
    }

    pub fn list_videos_for_playlist(&self, id: String, page_token: Option<String>) -> Result<(Vec<Video>, Option<String>)> {
        let mut search_params = vec![
            ("playlistId", id),
        ];
        if let Some(token) = page_token {
            search_params.push(("pageToken", token));
        }

        let (videos, page_token) = self.client.playlist_page(search_params)?;

        let videos = videos.into_iter()
            .map(|item| item.into_video().unwrap())
            .collect();
        Ok((videos, page_token))
    }
}
