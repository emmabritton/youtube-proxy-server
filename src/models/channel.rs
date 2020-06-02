use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Channel {
    id: String,
    title: String,
    thumbnail: String,
    #[serde(rename="youtubeVideoCount")]
    video_count: u64
}

impl Channel {
    pub fn new(id: String, title: String, thumbnail: String, video_count: u64) -> Channel {
        Channel {
            id,
            title,
            thumbnail,
            video_count
        }
    }
}