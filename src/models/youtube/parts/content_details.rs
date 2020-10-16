use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentDetails {
    related_playlists: RelatedPlaylists
}

#[derive(Deserialize, Debug)]
pub struct RelatedPlaylists {
    uploads: String
}

impl ContentDetails {
    pub fn get_upload_playlist_id(&self) -> Option<String> {
        return Some(self.related_playlists.uploads.clone())
    }
}