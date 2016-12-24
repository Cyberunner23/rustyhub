
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # rustyhub
//!
//! Rustyhub is a client for the Github v3 web API, the
//! intent is to create a client that is as complete as
//! possible. Once complete, it will offers an interface
//! to the public and enterprise API. It will also,
//! once complete, offer the capability for using all
//! supported authentication methods, media-types, etc.
//!
//! ## Usage
//! The crate is used by creating a
//! [Client](client/index.html) and then calling the
//! function associated to the desired endpoint with
//! appropriate arguments. Note: Each function borrows a
//! mutable [Client](client/index.html)
//!
//! ### Example
//!
//! ```rust,no_run
//!
//! extern crate rustyhub;
//!
//! use rustyhub::client::Client;
//! use rustyhub::activity::events;
//!
//! fn main() {
//!
//!     let github_token = "0000000000000000000000000000000000000000".to_string();
//!     let mut client = Client::new("rusyhub-UserAgent", Some(github_token));
//!
//!     let events = events::get_events(&mut client).unwrap();
//!
//! }
//!
//! ```


#![feature(proc_macro)]
//#![warn(missing_docs)]

extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

//TODO: Payloads
pub mod activity;
//TODO: Licenses once out of preview
pub mod miscellaneous;
//TODO: enterprise
//TODO: gists
//TODO: git_data
//TODO: integrations
//TODO: migration
//TODO: organizations
//TODO: projects
//TODO: pull_requests
//TODO: reactions
//TODO: repositories
//TODO: search
//TODO: users
//TODO: webhooks, include HMAC validation?

pub mod client;
pub mod common;
pub mod error;
pub mod utils;
