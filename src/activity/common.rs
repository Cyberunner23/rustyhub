// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

///Common response structures across various Activity endpoints.


#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Subscription {
    pub subscribed:     bool,
    pub ignored:        bool,
    pub reason:         Option<String>,
    pub created_at:     String,
    pub url:            String,
    //Returned in the notifications endpoints
    pub thread_url:     Option<String>,
    //Returned in the watching endpoints
    pub repository_url: Option<String>
}
