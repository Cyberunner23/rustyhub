
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::BTreeMap;
use serde_json;
use serde_json::Value;

use client::Client;
use error;


///Reference: https://developer.github.com/v3/activity/events/

/// The response of most event requests.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: String,
    public:     bool,
    payload:    BTreeMap<String, Value>,
    repo:       Repository,
    actor:      Actor,
    org:        Organization,
    created_at: String,
    id:         u64
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Repository {
    id:   u64,
    name: String,
    url:  String
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Actor {
    id:          u64,
    login:       String,
    gravatar_id: String,
    avatar_url:  String,
    url:         String
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Organization {
    id:          u64,
    login:       String,
    gravatar_id: String,
    avatar_url:  String,
    url:         String,
}


////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

pub fn get_events(client: &mut Client) -> Result<Vec<Event>, error::Error> {
    let mut response_events = try!(client.get("/events".to_string(), None));
    let response_str = try!(Client::response_to_string(&mut response_events));
    serde_json::from_str(& response_str[..]).map_err(error::Error::Parsing)
}


//TODO: tests
