use serde::Deserialize;
use crate::models::youtube::parts::kind::Kind;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    kind: Kind,
    channel_id: Option<String>,
    playlist_id: Option<String>,
    video_id: Option<String>,
}

impl Id {
    pub fn is_channel(&self) -> bool { self.kind.is_channel() }

    pub fn is_video(&self) -> bool { self.kind.is_video() }

    pub fn is_playlist(&self) -> bool { self.kind.is_playlist() }

    pub fn get_id(&self) -> String {
        return (if self.is_channel() {
            self.channel_id.as_ref().expect("Tried to get id for non channel")
        } else if self.is_video() {
            self.video_id.as_ref().expect("Tried to get id for non channel")
        } else if self.is_playlist() {
            self.playlist_id.as_ref().expect("Tried to get id for non channel")
        } else {
            panic!("Invalid kind");
        }).clone()
    }
}
