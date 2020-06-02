#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use anyhow::{Result, Context, Error};
use dotenv::dotenv;
use std::env;
use crate::key_manager::{KeyManager};
use crate::youtube_manager::{YoutubeManager};
use rocket::{State, Config, Request};
use rocket::config::{Environment};
use rocket_contrib::json::Json;
use std::collections::HashMap;
use rocket::response::{Responder, Debug};
use rocket::fairing::AdHoc;
use rocket::request::{FromRequest, Outcome};
use std::sync::{Arc, Mutex};
use rocket::http::Status;
use lazy_static::lazy_static;

mod endpoints;
mod models;
mod key_manager;
mod youtube_manager;

lazy_static! {
    static ref API_KEY: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

fn main() -> Result<()> {
    dotenv().ok();

    let hostname = env::var("HOSTNAME").unwrap_or(String::from("localhost"));
    let port: u16 = env::var("PORT").unwrap_or(String::from("3001")).parse().context("Invalid PORT").unwrap();
    let youtube_keys = env::var("YOUTUBE_API_KEYS").context("Invalid YOUTUBE_API_KEYS").unwrap().split(",").map(|item| item.to_string()).collect();
    let api_key = env::var("API_KEY").context("Invalid API_KEY").unwrap();

    *API_KEY.lock().unwrap() = api_key;

    let key_manager = KeyManager::new(youtube_keys);
    let youtube_manager = YoutubeManager::new(key_manager);

    println!("Starting youtube proxy server on {}:{}", hostname, port);

    let config = Config::build(Environment::active()?)
        .address(hostname)
        .port(port)
        .finalize()?;

    rocket::custom(config)
        .manage(youtube_manager)
        .mount("/", routes![alive, status, endpoints::search::channel, endpoints::search::video, endpoints::search::playlist])
        .launch();

    Ok(())
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
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        let api_key = API_KEY.lock().unwrap().clone();
        if keys.len() >= 1 && keys[0] == api_key {
            Outcome::Success(ApiKey {})
        } else {
            Outcome::Failure((Status::InternalServerError, format!("Server Error")))
        }
    }
}