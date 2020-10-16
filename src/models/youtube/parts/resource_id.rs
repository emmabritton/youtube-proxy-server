use serde::Deserialize;
use crate::models::youtube::parts::kind::Kind;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceId {
    pub kind: Kind,
    pub video_id: String
}