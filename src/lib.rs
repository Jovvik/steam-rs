//! # steam-rs: Safe bindings for the Steam Web API
//!
//! The `steam-rs` crate provides convenient Rust bindings for the Steam Web API.
//! This crate provides safe and convenient Rust bindings for the Steam Web API.
//!
//! ### Warning!
//! This crate is currently a work in progress, so please expect breaking changes and instability. Please be careful when using this! **This is not production ready!**

use std::num::NonZeroU16;

use reqwest::{Client, StatusCode};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Error};
use reqwest_retry::{
    default_on_request_failure, policies::ExponentialBackoff, RetryTransientMiddleware, Retryable,
    RetryableStrategy,
};

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

struct SteamRetryableStrategy;

impl RetryableStrategy for SteamRetryableStrategy {
    fn handle(&self, res: &Result<reqwest::Response, Error>) -> Option<Retryable> {
        match res {
            Ok(success) => {
                let enhance_your_calm: StatusCode = StatusCode::from_u16(420).unwrap();
                let status = success.status();
                if status.is_server_error() {
                    Some(Retryable::Transient)
                } else if status.is_client_error()
                    && status != StatusCode::REQUEST_TIMEOUT
                    && status != StatusCode::TOO_MANY_REQUESTS
                    && status != enhance_your_calm
                {
                    Some(Retryable::Fatal)
                } else if status.is_success() {
                    None
                } else if status == StatusCode::REQUEST_TIMEOUT
                    || status == StatusCode::TOO_MANY_REQUESTS
                    || status == enhance_your_calm
                {
                    Some(Retryable::Transient)
                } else {
                    Some(Retryable::Fatal)
                }
            }
            Err(error) => default_on_request_failure(error),
        }
    }
}

pub struct Steam {
    pub api_key: String,
    pub client: ClientWithMiddleware,
}

impl Steam {
    pub fn new(api_key: &str, client: Client, policy: ExponentialBackoff) -> Steam {
        let middleware =
            RetryTransientMiddleware::new_with_policy_and_strategy(policy, SteamRetryableStrategy);
        let client = ClientBuilder::new(client).with(middleware).build();
        Steam {
            api_key: api_key.to_string(),
            client,
        }
    }
}
