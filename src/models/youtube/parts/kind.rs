use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Kind(String);

impl Kind {
    pub fn is_channel(&self) -> bool { self.0 == "youtube#channel" }

    pub fn is_video(&self) -> bool { self.0 == "youtube#video" }

    pub fn is_playlist(&self) -> bool { self.0 == "youtube#playlist" }
}