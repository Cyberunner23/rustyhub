// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Gitignore
//!
//! These are the responses and API call functions related
//! to the gitignore endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/activity/watching/

use hyper::header::{Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};

use client::Client;
use error;
use utils;

///The response of the non-raw single template request.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GitignoreTemplate {
    pub name:   String,
    pub source: String
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait GitignoreExt {

    /// \[[Reference](https://developer.github.com/v3/gitignore/#listing-available-templates)\]
    /// Returns the list available templates when creating a
    /// repository.
    /// ## Endpoint:
    /// GET /gitignore/templates
    fn get_gitignore_templates(&mut self) -> Result<Vec<String>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gitignore/#get-a-single-template)\]
    /// Returns a single template.
    /// ## Endpoint:
    /// GET /gitignore/templates/:name
    /// ## Parameters:
    /// * `name`: Name of/the language template requested
    fn get_gitignore_templates_name(&mut self, name: &String) -> Result<GitignoreTemplate, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gitignore/#get-a-single-template)\]
    /// Returns a single template in raw form.
    /// ## Endpoint:
    /// GET /gitignore/templates/:name
    /// ## Parameters:
    /// * `name`: Name of/the language template requested
    fn get_gitignore_templates_name_raw(&mut self, name: &String) -> Result<String, error::Error>;
}

impl GitignoreExt for Client {

    fn get_gitignore_templates(&mut self) -> Result<Vec<String>, error::Error> {
        utils::request_endpoint(self, "/gitignore/templates".into())
    }

    fn get_gitignore_templates_name(&mut self, name: &String) -> Result<GitignoreTemplate, error::Error> {
        utils::request_endpoint(self, format!("/gitignore/templates/{}", name))
    }

    fn get_gitignore_templates_name_raw(&mut self, name: &String) -> Result<String, error::Error> {

        let mut header = self.get_default_headers();
        header.remove::<Accept>();
        header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3.raw".to_string()), vec![]))]));

        utils::request_endpoint_with_headers(self, format!("/gitignore/templates/{}", name), Some(header))
    }
}

//TODO: TESTS
