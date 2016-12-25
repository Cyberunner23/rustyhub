
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


use serde_json;
use serde;

use client::Client;
use error;

pub fn request_endpoint<D:serde::Deserialize>(client: &Client, 
                                              endpoint: String) -> Result<D, error::Error> {
    let mut response     = try!(client.get(endpoint, None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}
