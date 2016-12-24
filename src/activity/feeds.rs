// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use serde_json;

use client::Client;
use error;

///Reference: https://developer.github.com/v3/activity/feeds/

///Response to feeds requests
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Feeds {
    pub timeline_url:                   String,
    pub user_url:                       String,
    pub current_user_public_ur:         Option<String>,
    pub current_user_url:               Option<String>,
    pub current_user_actor_url:         Option<String>,
    pub current_user_organization_url:  Option<String>,
    pub current_user_organization_urls: Option<Vec<String>>,
    #[serde(rename = "_links")]
    pub links: Links
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Links {
    pub timeline:                   FeedsElem,
    pub user:                       FeedsElem,
    pub current_user_public:        Option<FeedsElem>,
    pub current_user:               Option<FeedsElem>,
    pub current_user_actor:         Option<FeedsElem>,
    pub current_user_organization:  Option<FeedsElem>,
    pub current_user_organizations: Option<FeedsElem>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FeedsElem {
    pub href: String,
    #[serde(rename = "type")]
    pub mime_type: String
}


////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

pub fn get_feeds(client: &mut Client) -> Result<Feeds, error::Error> {
    let mut response     = try!(client.get("/feeds".to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

//TODO:TESTS
