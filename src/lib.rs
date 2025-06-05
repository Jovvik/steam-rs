//! # steam-rs: Safe bindings for the Steam Web API
//!
//! The `steam-rs` crate provides convenient Rust bindings for the Steam Web API.
//! This crate provides safe and convenient Rust bindings for the Steam Web API.
//!
//! ### Warning!
//! This crate is currently a work in progress, so please expect breaking changes and instability. Please be careful when using this! **This is not production ready!**

use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::RetryTransientMiddleware;

pub mod econ_service;
pub mod player_service;
pub mod published_file_service;
pub mod site_license_service;
pub mod steam_apps;
pub mod steam_economy;
pub mod steam_id;
pub mod steam_news;
pub mod steam_remote_storage;
pub mod steam_user;
pub mod steam_user_auth;
pub mod steam_user_stats;
pub mod steam_webapi_util;

pub mod errors;
mod macros; // This remains private

const BASE: &str = "https://api.steampowered.com";

pub use reqwest_retry::policies as retry_policies;

pub struct Steam {
    api_key: String,
    client: ClientWithMiddleware,
}

impl Steam {
    pub fn new(api_key: &str, policy: retry_policies::ExponentialBackoff) -> Steam {
        let middleware = RetryTransientMiddleware::new_with_policy(policy);
        let client = ClientBuilder::new(Client::new()).with(middleware).build();
        Steam {
            api_key: api_key.to_string(),
            client,
        }
    }
}
