use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    video_count: String
}

impl Default for Stats {
    fn default() -> Self {
        return Stats {
            video_count: String::from("0")
        };
    }
}

impl Stats {
    pub fn get_video_count(&self) -> u64 {
        return self.video_count.parse().unwrap_or(0);
    }
}
