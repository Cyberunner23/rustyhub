// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Rate Limit
//!
//! These are the responses and API call functions related
//! to the rate_limit endpoint of the API.
//!
//! Reference: https://developer.github.com/v3/rate_limit/

use client::Client;
use error;
use utils;

/// The response to rate_limit endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RateLimit {
    pub resources: RateLimitResources
    //rate omitted since it is deprecated and will ne removed in the next version of the API
}

///Sub-component of the rate_limit response.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RateLimitResources {
    pub core:   RateLimitElement,
    pub search: RateLimitElement
}

///Sub-component of the rate_limit response.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RateLimitElement {
    pub limit:     u64,
    pub remaining: u64,
    pub reset:     u64
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait RateLimitExt {
    /// \[[Reference](https://developer.github.com/v3/rate_limit/#get-your-current-rate-limit-status)\]
    /// Returns the current rate limit status.
    /// ## Endpoint:
    /// GET /rate_limit
    fn get_rate_limit(&mut self) -> Result<RateLimit, error::Error>;
}

impl RateLimitExt for Client {
    fn get_rate_limit(&mut self) -> Result<RateLimit, error::Error> {
        utils::request_endpoint(self, "/rate_limit".into())
    }
}

//TODO: TESTS
