// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use client::Client;
use error;
use utils;

///Reference: https://developer.github.com/v3/meta/

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Meta {
    pub verifiable_password_authentication: bool,
    pub github_services_sha: String,
    pub hooks:    Vec<String>,
    pub pages:    Vec<String>,
    pub importer: Vec<String>,
}

////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

pub fn get_meta(client: &mut Client) -> Result<Meta, error::Error> {
    utils::request_endpoint(client, "/meta".into())
}

//TODO: TESTS
