
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Error as IOError;

/// Represents all possible errors than can occur while using rustyhub.
#[derive(Debug)]
pub enum Error {

    /// Error returned by github for invalid API usage.
    Github(GithubError),
    /// Error returned by hyper when parsing HTTP streams.
    HTTP(::hyper::Error),
    /// Error returned by serde when parsing JSON data from a string.
    Parsing(::serde_json::Error),
    /// Errors returned by functions in std
    STDIO(IOError),
}

#[derive(Debug, Deserialize)]
pub struct GithubError {
    message:           String,
    documentation_url: Option<String>,
    errors:            Option<Vec<GithubErrorErrors>>
}

#[derive(Debug, Deserialize)]
pub struct GithubErrorErrors{
    resources: String,
    field:     String,
    code:      String
}

//TODO: Tests
