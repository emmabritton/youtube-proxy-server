use anyhow::{Result, Error};
use rocket::request::{FromParam};
use rocket::http::RawStr;

pub enum ContentType {
    CHANNEL,
    VIDEO,
    PLAYLIST
}

impl ContentType {
    pub fn from_string(value: &str) -> Result<ContentType> {
        match value {
            "channel" => Ok(ContentType::CHANNEL),
            "video" => Ok(ContentType::VIDEO),
            "playlist" => Ok(ContentType::PLAYLIST),
            _ => Err(Error::msg(format!("Invalid type: {}", value)))
        }
    }
}

impl <'v> FromParam<'v> for ContentType {
    type Error = String;

    fn from_param(param: &'v RawStr) -> Result<Self, Self::Error> {
        let value = param.as_str();
        match value {
            "channel" => Ok(ContentType::CHANNEL),
            "video" => Ok(ContentType::VIDEO),
            "playlist" => Ok(ContentType::PLAYLIST),
            _ => Err(format!("Invalid type: {}", value))
        }
    }
}
