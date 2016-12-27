
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Events
//!
//! These are the responses and API call functions related
//! to the event endpoints of the API.
//!
//! NOTE: `GET /repos/:owner/:repo/issues/events` is in
//! issues::events
//!
//! Reference: https://developer.github.com/v3/activity/events/

use std::collections::BTreeMap;
use serde_json::Value;

use common::{Repository, User};
use client::Client;
use error;
use utils;

/// The response of most event requests.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    pub public:     bool,
    pub payload:    BTreeMap<String, Value>,
    pub repo:       Repository,
    pub actor:      User,
    pub org:        Option<Organization>,
    pub created_at: String,
    pub id:         u64
}

/// Sub-component of the Event response
//TODO?: move to common?
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Organization {
    pub id:          u64,
    pub login:       String,
    pub gravatar_id: String,
    pub avatar_url:  String,
    pub url:         String,
}


////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait EventsExt {

    /// \[[Reference](https://developer.github.com/v3/activity/events/#list-public-events)\]
    /// Returns the list of public events.
    /// ## Endpoint:
    /// GET /events
    fn get_events(&mut self) -> Result<Vec<Event>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/events/#list-repository-events)\]
    /// Returns the list of repository events.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/events
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_repo_events(&mut self, owner: String, repo: String) -> Result<Vec<Event>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/events/#list-public-events-for-a-network-of-repositories)\]
    /// Returns a list public events for a network of repositories.
    /// ## Endpoint:
    /// GET /networks/:owner/:repo/events
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_networks_owner_repo_events(&mut self, owner: String, repo: String) -> Result<Vec<Event>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/events/#list-public-events-for-an-organization)\]
    /// Returns a list public events for an organization.
    /// ## Endpoint:
    /// GET /orgs/:org/events
    /// ## Parameters:
    /// * `org`: Name of the organization
    fn get_orgs_org_events(&mut self, org: String) -> Result<Vec<Event>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/events/#list-events-that-a-user-has-received)\]
    /// Returns a list events that a user has received.
    /// ## Endpoint:
    /// GET /users/:username/received_events
    /// ## Parameters:
    /// * `username`: Name of the user
    fn get_users_username_received_events(&mut self, username: String) -> Result<Vec<Event>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/events/#list-public-events-that-a-user-has-received)\]
    /// Returns a list public events that a user has received.
    /// ## Endpoint:
    /// GET /users/:username/received_events/public
    /// ## Parameters:
    /// * `username`: Name of the user
    fn get_users_username_received_events_public(&mut self, username: String) -> Result<Vec<Event>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/events/#list-events-performed-by-a-user)\]
    /// Returns a list events performed by a user.
    /// ## Endpoint:
    /// GET /users/:username/events
    /// ## Parameters:
    /// * `username`: Name of the user
    fn get_users_username_events(&mut self, username: String) -> Result<Vec<Event>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/events/#list-public-events-performed-by-a-user)\]
    /// Returns a list public events performed by a user.
    /// ## Endpoint:
    /// GET /users/:username/events/public
    /// ## Parameters:
    /// * `username`: Name of the user
    fn get_users_username_events_public(&mut self, username: String) -> Result<Vec<Event>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/events/#list-events-for-an-organization)\]
    /// Returns a list events for an organization.
    /// ## Endpoint:
    /// GET /users/:username/events/orgs/:org
    /// ## Parameters:
    /// * `username`: Name of the user
    /// * `org`: Name of the organization
    fn get_users_username_events_orgs_org(&mut self, username: String, org: String) -> Result<Vec<Event>, error::Error>;
}

impl EventsExt for Client {

    fn get_events(&mut self) -> Result<Vec<Event>, error::Error> {
        utils::request_endpoint(self, "/events".into())
    }

    fn get_repo_events(&mut self, owner: String, repo: String) -> Result<Vec<Event>, error::Error> {
        utils::request_endpoint(self, format!("/repos/{}/{}/events", owner, repo))
    }

    fn get_networks_owner_repo_events(&mut self, owner: String, repo: String) -> Result<Vec<Event>, error::Error> {
        utils::request_endpoint(self, format!("/networks/{}/{}/events", owner, repo))
    }

    fn get_orgs_org_events(&mut self, org: String) -> Result<Vec<Event>, error::Error> {
        utils::request_endpoint(self, format!("/orgs/{}/events", org))
    }

    fn get_users_username_received_events(&mut self, username: String) -> Result<Vec<Event>, error::Error> {
        utils::request_endpoint(self, format!("/users/{}/received_events", username))
    }

    fn get_users_username_received_events_public(&mut self, username: String) -> Result<Vec<Event>, error::Error> {
        utils::request_endpoint(self, format!("/users/{}/received_events/public", username))
    }

    fn get_users_username_events(&mut self, username: String) -> Result<Vec<Event>, error::Error> {
        utils::request_endpoint(self, format!("/users/{}/events", username))
    }

    fn get_users_username_events_public(&mut self, username: String) -> Result<Vec<Event>, error::Error> {
        utils::request_endpoint(self, format!("/users/{}/events/public", username))
    }

    fn get_users_username_events_orgs_org(&mut self, username: String, org: String) -> Result<Vec<Event>, error::Error> {
        utils::request_endpoint(self, format!("/users/{}/events/orgs/{}", username, org))
    }
}

//TODO: tests
