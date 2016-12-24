
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
    pub event_type: String,
    pub public:     bool,
    pub payload:    BTreeMap<String, Value>,
    pub repo:       Repository,
    pub actor:      Actor,
    pub org:        Organization,
    pub created_at: String,
    pub id:         u64
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Repository {
    pub id:   u64,
    pub name: String,
    pub url:  String
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Actor {
    pub id:          u64,
    pub login:       String,
    pub gravatar_id: String,
    pub avatar_url:  String,
    pub url:         String
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Organization {
    pub id:          u64,
    pub login:       String,
    pub gravatar_id: String,
    pub avatar_url:  String,
    pub url:         String,
}


////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

///Reference: https://developer.github.com/v3/activity/events/#list-public-events
pub fn get_events(client: &mut Client) -> Result<Vec<Event>, error::Error> {
    let mut response     = try!(client.get("/events".to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/events/#list-repository-events
pub fn get_repo_events(client: &mut Client, owner: String, repo: String) -> Result<Vec<Event>, error::Error> {
    let mut response     = try!(client.get(format!("/repos/{}/{}/events", owner, repo).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

//TODO: GET /repos/:owner/:repo/issues/events
///Reference: https://developer.github.com/v3/activity/events/#list-issue-events-for-a-repository

///Reference: https://developer.github.com/v3/activity/events/#list-public-events-for-a-network-of-repositories
pub fn get_networks_owner_repo_events(client: &mut Client, owner: String, repo: String) -> Result<Vec<Event>, error::Error> {
    let mut response     = try!(client.get(format!("/networks/{}/{}/events", owner, repo).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/events/#list-public-events-for-an-organization
pub fn get_orgs_org_events(client: &mut Client, org: String) -> Result<Vec<Event>, error::Error> {
    let mut response     = try!(client.get(format!("/orgs/{}/events", org).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/events/#list-events-that-a-user-has-received
pub fn get_users_username_received_events(client: &mut Client, username: String) -> Result<Vec<Event>, error::Error> {
    let mut response     = try!(client.get(format!("/users/{}/received_events", username).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/events/#list-public-events-that-a-user-has-received
pub fn get_users_username_received_events_public(client: &mut Client, username: String) -> Result<Vec<Event>, error::Error> {
    let mut response     = try!(client.get(format!("/users/{}/received_events/public", username).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/events/#list-events-performed-by-a-user
pub fn get_users_username_events(client: &mut Client, username: String) -> Result<Vec<Event>, error::Error> {
    let mut response     = try!(client.get(format!("/users/{}/events", username).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/events/#list-public-events-performed-by-a-user
pub fn get_users_username_events_public(client: &mut Client, username: String) -> Result<Vec<Event>, error::Error> {
    let mut response     = try!(client.get(format!("/users/{}/events/public", username).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/events/#list-events-for-an-organization
pub fn get_users_username_events_orgs_org(client: &mut Client, username: String, org: String) -> Result<Vec<Event>, error::Error> {
    let mut response     = try!(client.get(format!("/users/{}/events/orgs/{}", username, org).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}



//TODO: tests
