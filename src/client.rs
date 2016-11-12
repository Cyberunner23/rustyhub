
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use hyper::header::Headers;

static DEFAULT_API_URL: &'static str = "https://api.github.com/";

#[derive(Clone, Debug)]
pub struct Client {
    pub api_url:    String,
    pub user_agent: String,
    pub auth_token: Option<String>,
    pub headers:    Headers
}