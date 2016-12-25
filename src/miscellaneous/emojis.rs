// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Emojis
//!
//! These are the responses and API call functions related
//! to the emojis endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/emojis/

use std::collections::BTreeMap;
use serde_json;
use serde_json::Value;

use client::Client;
use error;

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait EmojisExt {

    //TODO: Need to somehow clean up this error reporting
    /// \[[Reference](https://developer.github.com/v3/activity/watching/#list-watchers)\]
    /// Returns a map of emojis.
    /// ## Endpoint:
    /// GET /emojis
    /// ## Returns:
    /// * Returns Ok(None) if the conversion to object
    /// failed, please create an issue with the debug
    /// output of the error is this ever occurs.
    /// * The value for the particular key will be "" if
    /// the conversion to String failed, please create an
    /// issue with the debug output of the error if this ever
    /// occurs.
    fn get_emojis(&mut self) -> Result<Option<BTreeMap<String, String>>, error::Error>;
}

impl EmojisExt for Client {
    fn get_emojis(&mut self) -> Result<Option<BTreeMap<String, String>>, error::Error> {

        let mut response     = try!(self.get("/emojis".to_string(), None));
        let     response_str = try!(Client::response_to_string(&mut response));

        let response_val: Value = try!(serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing));
        let response_obj: BTreeMap<String, Value> = match response_val.as_object(){
            Some(obj) => obj.clone(),
            None      => return Ok(None)
        };

        let mut emojis: BTreeMap<String, String> = BTreeMap::new();
        for (key, val) in response_obj {
            emojis.insert(key, val.as_str().unwrap_or_else(|| "").to_string().clone());
        }

        Ok(Some(emojis))
    }
}

//TODO: TESTS
