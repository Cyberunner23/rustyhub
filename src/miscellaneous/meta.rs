// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use serde_json;

use client::Client;
use error;

///Reference: https://developer.github.com/v3/meta/

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Meta {
    verifiable_password_authentication: bool,
    github_services_sha: String,
    hooks:    Vec<String>,
    pages:    Vec<String>,
    importer: Vec<String>,
}

////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

pub fn get_meta(client: &mut Client) -> Result<Meta, error::Error> {
    match serde_json::from_str(&try!(Client::response_to_string(&mut try!(client.get("/meta".to_string(), None))))[..]) {
        Ok(meta) => Ok(meta),
        Err(err) => Err(error::Error::Parsing(err))
    }
}

//TODO: TESTS
