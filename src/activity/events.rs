
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::BTreeMap;
use serde_json::Value;

use client::Client;
use error::Error;


///Reference: https://developer.github.com/v3/activity/events/

/// The response of most event requests.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: String,
    public:     bool,
    #[serde(skip_deserializing)]
    payload:    Payload,
    repo:       Repository,
    actor:      Actor,
    org:        Organization,
    created_at: String,
    id:         u64
}


/// Each event type has a unique payload structure, for that matter payload data is stored as an object map.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Payload {
    payload: BTreeMap<String, Value>
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

/*pub fn get_events(client: &Client) -> Result<Vec<Event>, Error> {



    //copy of json
    //parse json to Value
    //parse json to Vec<Event>
    //for each event in Value, extract payload -> ValueMap
    //inject in Event

}*/


//TODO: tests