// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use hyper::header::{Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde_json;

use client::Client;
use error;

///Reference: https://developer.github.com/v3/gitignore/

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GitignoreTemplate {
    name:   String,
    source: String
}

////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

///Reference: https://developer.github.com/v3/gitignore/#listing-available-templates
pub fn get_gitignore_templates(client: &mut Client) -> Result<Vec<String>, error::Error> {
    let mut response     = try!(client.get("/gitignore/templates".to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/gitignore/#get-a-single-template
pub fn get_gitignore_templates_name(client: &mut Client, name: &String) -> Result<GitignoreTemplate, error::Error> {
    let mut response     = try!(client.get(format!("/gitignore/templates/{}", name).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

pub fn get_gitignore_templates_name_raw(client: &mut Client, name: &String) -> Result<String, error::Error> {

    let mut header = client.get_default_header();
    header.remove::<Accept>();
    header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3.raw".to_string()), vec![]))]));

    let mut response     = try!(client.get(format!("/gitignore/templates/{}", name).to_string(), Some(header)));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

//TODO: TESTS
