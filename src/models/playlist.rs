use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    id: String,
    title: String,
    thumbnail: String,
    channel_id: String,
    channel_title: String
}

impl Playlist {
    pub fn new(id: String, title: String, thumbnail: String, channel_id: String, channel_title: String) -> Self {
        Playlist {
            id,
            title,
            thumbnail,
            channel_id,
            channel_title
        }
    }
}