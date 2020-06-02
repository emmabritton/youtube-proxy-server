#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use anyhow::{Result, Context};
use dotenv::dotenv;
use std::env;
use crate::key_manager::{KeyManager};
use crate::youtube_manager::{YoutubeManager};
use rocket::{State, Config};
use rocket::config::{Environment};
use rocket_contrib::json::Json;
use std::collections::HashMap;
use rocket::response::{Responder, Debug};

mod endpoints;
mod models;
mod key_manager;
mod youtube_manager;

fn main() -> Result<()> {
    dotenv().ok();

    let hostname = env::var("HOSTNAME").unwrap_or(String::from("localhost"));
    let port: u16 = env::var("PORT").unwrap_or(String::from("3001")).parse().context("Invalid PORT").unwrap();
    let youtube_keys = env::var("YOUTUBE_API_KEYS").context("Invalid YOUTUBE_API_KEYS").unwrap().split(",").map(|item| item.to_string()).collect();
    let api_key = env::var("API_KEY").context("Invalid API_KEY").unwrap();

    let key_manager = KeyManager::new(youtube_keys);
    let youtube_manager = YoutubeManager::new(key_manager, api_key);

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
fn status(youtube_manager: State<YoutubeManager>) -> Json<HashMap<usize, usize>> {
    Json(youtube_manager.get_key_status())
}

