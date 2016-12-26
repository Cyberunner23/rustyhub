// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO?: Move to mod.rs

//! # Auth
//!
//! Represents the different methods of authentication
//! that can be used for the API

#[derive(Clone, Debug, PartialEq)]
pub enum Auth {
    /// Use the Github API un-authenticated
    NoAuth,
    /// Use the Github API authenticated with an OAuth2
    /// token.
    /// ## Parameters
    /// * OAuth token.
    OAuth2Token(String),
    /// Use the Github API authenticated with an OAuth2
    /// key/secret.
    /// ## Parameters
    /// * Client ID.
    /// * Client secret.
    OAuth2KeySecret(String, String),
    /// Use the Github API authenticated with a username
    /// password and optionally with a a two-factor
    /// OTP
    /// # Parameters
    /// * Username
    /// * Password
    /// * Optional OTP
    Basic(String, String, Option<String>)
}
