// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Meta
//!
//! These are the responses and API call functions related
//! to the meta endpoint of the API.
//!
//! Reference: https://developer.github.com/v3/meta/

use client::Client;
use error;
use utils;

///Response returned by the meta endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Meta {
    /// Whether authentication with username and password
    /// is supported.
    pub verifiable_password_authentication: bool,
    /// The currently-deployed SHA of github-services.
    pub github_services_sha: String,
    /// An Array of IP addresses that incoming service hooks
    /// will originate from on GitHub.com.
    pub hooks:    Vec<String>,
    /// An Array of IP addresses specifying the Git servers
    /// for GitHub.com.
    pub git:      Vec<String>,
    /// An Array of IP addresses specifying the A records
    /// for GitHub Pages.
    pub pages:    Vec<String>,
    /// An Array of IP addresses specifying the addresses
    /// that source imports will originate from on
    /// GitHub.com.
    pub importer: Vec<String>,
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait MetaExt {

    /// \[[Reference](https://developer.github.com/v3/markdown/#render-an-arbitrary-markdown-document)\]
    /// Returns information about GitHub
    /// ## Endpoint:
    /// GET /meta
    fn get_meta(&mut self) -> Result<Meta, error::Error>;
}

impl MetaExt for Client {
    fn get_meta(&mut self) -> Result<Meta, error::Error> {
        utils::request_endpoint(self, "/meta".into())
    }
}

//TODO: TESTS
