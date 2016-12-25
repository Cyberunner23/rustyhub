// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Common
//!
//! These are the responses that are in common or have
//! very high similarity across the Activity endpoints.


/// Represents a subscription, this is returned by
/// some endpoints in
/// [Notifications](../notifications/index.html)
/// and
/// [Watching](../watching/index.html)
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Subscription {

    pub subscribed:     bool,
    pub ignored:        bool,
    pub reason:         Option<String>,
    pub created_at:     String,
    pub url:            String,
    /// URL to the notification thread, set in the
    /// notifications endpoints
    pub thread_url:     Option<String>,
    /// URL to the repository, set in the watching
    /// endpoints
    pub repository_url: Option<String>
}
