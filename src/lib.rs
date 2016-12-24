
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![feature(proc_macro)]

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
