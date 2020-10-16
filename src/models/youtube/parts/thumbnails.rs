use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Thumbnails {
    high: Option<Thumbnail>,
    medium: Option<Thumbnail>,
    default: Thumbnail,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnail {
    url: String
}

impl Thumbnails {
    pub fn empty() -> Thumbnails {
        Thumbnails {
            default: { Thumbnail { url: String::new() } },
            medium: None,
            high: None,
        }
    }

    pub fn get_url(&self) -> String {
        (if let Some(high) = &self.high {
            &high.url
        } else if let Some(med) = &self.medium {
            &med.url
        } else {
            &self.default.url
        }).clone()
    }
}
