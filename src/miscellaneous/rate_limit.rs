// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use serde_json;

use client::Client;
use error;

///Reference: https://developer.github.com/v3/rate_limit/

/// The response to GET /rate_limit
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RateLimit {
    pub resources: RateLimitResources
    //rate omitted since it is deprecated and will ne removed in the next version of the API
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RateLimitResources {
    pub core:   RateLimitElement,
    pub search: RateLimitElement
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RateLimitElement {
    pub limit:     u64,
    pub remaining: u64,
    pub reset:     u64
}

////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

pub fn get_rate_limit(client: &mut Client) -> Result<RateLimit, error::Error> {
    let mut response     = try!(client.get("/rate_limit".to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

//TODO: TESTS
