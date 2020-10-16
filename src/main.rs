#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use anyhow::{Result, Context};
use dotenv::dotenv;
use std::env;
use crate::key_manager::{KeyManager};
use crate::youtube_manager::YoutubeManager;
use crate::youtube_client::YOUTUBE_URL;
use rocket::{State, Config, Request, Rocket};
use rocket::config::{Environment};
use rocket_contrib::json::Json;
use std::collections::HashMap;
use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;

mod endpoints;
mod models;
mod key_manager;
mod youtube_manager;
mod date_util;
mod youtube_client;

fn main() -> Result<()> {
    dotenv().ok();

    let hostname = env::var("HOSTNAME").unwrap_or(String::from("localhost"));
    let port: u16 = env::var("PORT").unwrap_or(String::from("3001")).parse().context("Invalid PORT").unwrap();
    let proxy = env::var("PROXY").ok();
    let youtube_keys = env::var("YOUTUBE_API_KEYS").context("Invalid YOUTUBE_API_KEYS").unwrap().split(",").map(|item| item.to_string()).collect();
    let api_key = if env::var_os("API_KEY").is_some() {
        let key = env::var("API_KEY").context("Invalid API_KEY").unwrap();
        Some(key)
    } else {
        None
    };

    let key_manager = KeyManager::new(youtube_keys);
    let youtube_manager = YoutubeManager::new(key_manager, YOUTUBE_URL.to_string(), &proxy);

    println!("Starting youtube proxy server on {}:{}", hostname, port);

    if let Some(proxy) = proxy {
        println!("Proxying requests via {}", proxy);
    }

    let config = Config::build(Environment::active()?)
        .address(hostname)
        .port(port)
        .finalize()?;

    make_rocket(config, api_key, youtube_manager).launch();

    Ok(())
}

fn make_rocket(config: Config, api_key: Option<String>, youtube_manager: YoutubeManager) -> Rocket {
    return rocket::custom(config)
        .manage(youtube_manager)
        .manage(api_key)
        .mount("/", routes![alive, status,
            endpoints::search::channel, endpoints::search::video, endpoints::search::playlist,
            endpoints::single::channel, endpoints::single::video, endpoints::single::playlist,
            endpoints::videos::get_most_recent_videos_for_channel, endpoints::videos::get_videos_for_playlist]);
}

#[get("/alive")]
fn alive() -> &'static str {
    "OK"
}

#[get("/v1/admin/status")]
fn status(youtube_manager: State<YoutubeManager>, _api_key: ApiKey) -> Json<HashMap<usize, usize>> {
    Json(youtube_manager.get_key_status())
}

pub struct ApiKey {}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let api_key_outcome = request.guard::<State<Option<String>>>().unwrap();
        if api_key_outcome.is_none() { return Outcome::Success(ApiKey {}); }

        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        let api_key = api_key_outcome.as_ref().unwrap().to_string();
        if keys.len() >= 1 && keys[0] == api_key {
            Outcome::Success(ApiKey {})
        } else {
            Outcome::Failure((Status::Unauthorized, format!("Server Error")))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito;
    use rocket::local::Client;
    use lazy_static::lazy_static;
    use mockito::{mock, Matcher};
    use std::path::PathBuf;

    lazy_static! {
        static ref DEFAULT_KEYS: Vec<&'static str> = vec!["key1", "key2"];
        static ref TEST_API_KEY: Option<String> = Some(String::from("test"));
    }

    //Make client
    fn make_client(keys: Vec<&'static str>, api_key: Option<String>) -> Client {
        dotenv().ok();
        let key_manager = KeyManager::new_test(keys);
        let youtube_manager = YoutubeManager::new(key_manager, mockito::server_url().clone(), &env::var("PROXY").ok());
        let client = Client::new(make_rocket(Config::development(), api_key, youtube_manager)).expect("valid rocket instance");
        client
    }

    #[test]
    fn test_alive() {
        //GIVEN client with default keys
        let client = make_client(DEFAULT_KEYS.clone(), None);
        //WHEN making /alive request
        let mut response = client.get("/alive").dispatch();
        //THEN response is ok
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("OK".into()));
    }

    #[test]
    fn test_auth_endpoints() {
        //GIVEN client with default keys and the api key is set
        let client = make_client(DEFAULT_KEYS.clone(), TEST_API_KEY.clone());
        //WHEN making all requests that require api keys
        let status = client.get("/v1/admin/status").dispatch();
        let search_channel = client.get("/v1/search/channel?q=test").dispatch();
        let search_video = client.get("/v1/search/video?q=test").dispatch();
        let search_playlist = client.get("/v1/search/playlist?q=test").dispatch();
        //THEN all requests should return unauth
        assert_eq!(status.status(), Status::Unauthorized);
        assert_eq!(search_channel.status(), Status::Unauthorized);
        assert_eq!(search_video.status(), Status::Unauthorized);
        assert_eq!(search_playlist.status(), Status::Unauthorized);
    }

    #[test]
    fn test_status_1_full_key() {
        //GIVEN client with one key
        let client = make_client(vec!["key1"], None);
        //WHEN making /v1/admin/status request
        let mut response = client.get("/v1/admin/status").dispatch();
        //THEN response is as expected
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"0":10000}"#.into()));
    }

    #[test]
    fn test_search_channel() {
        run_resource_test("search_result_channel.json", r"/search\?.*", || {
            //GIVEN client with default keys
            let client = make_client(DEFAULT_KEYS.clone(), None);
            //WHEN search for channels by 'test'
            let mut response = client.get("/v1/search/channel?q=test").dispatch();
            //THEN check results are list of channels
            let expected = load_test_file("search_expected_channel.json");
            assert_eq!(response.status().code, 200);
            assert_eq!(response.body_string(), Some(expected));
        });
    }

    fn run_resource_test(file: &'static str, path: &'static str, test: impl Fn() -> ()) {
        let json = load_test_file(file);
        let _mock = mock("GET", Matcher::Regex(path.to_string())).with_body(json.clone()).create();
        test();
        _mock.assert();
    }

    fn load_test_file(file: &'static str) -> String {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push(format!("test/resources/{}", file));
        return std::fs::read_to_string(file_path).unwrap();
    }
}