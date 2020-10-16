use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    id: String,
    title: String,
    thumbnail: String,
    #[serde(rename = "youtubeVideoCount", skip_serializing_if = "Option::is_none")]
    video_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_playlist_id: Option<String>
}

impl Channel {
    pub fn new(id: String, title: String, thumbnail: String, video_count: Option<u64>, upload_playlist_id: Option<String>) -> Self {
        Channel { id, title, thumbnail, video_count, upload_playlist_id }
    }
}

impl Channel {
    pub fn get_all_videos_playlist_id(&self) -> Option<String> {
        self.upload_playlist_id.clone()
    }
}