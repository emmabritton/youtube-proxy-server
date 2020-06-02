use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    id: String,
    title: String,
    date: String,
    thumbnail: String,
    channel_id: String,
    channel_title: String,
    description: Option<String>,
}

impl Video {
    pub fn new(id: String, title: String, date: String, thumbnail: String, channel_id: String, channel_title: String, description: Option<String>) -> Self {
        Video {
            id,
            title,
            date,
            thumbnail,
            channel_id,
            channel_title,
            description,
        }
    }
}