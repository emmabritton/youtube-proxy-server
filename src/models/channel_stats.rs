use serde::Serialize;
use crate::models::youtube::parts::stats::Stats;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelStats {
    youtube_video_count: u64
}

impl From<Stats> for ChannelStats {
    fn from(stats: Stats) -> Self {
        ChannelStats {
            youtube_video_count: stats.get_video_count()
        }
    }
}