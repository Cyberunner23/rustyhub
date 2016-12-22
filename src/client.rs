
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


/// The default API URL.
static DEFAULT_API_URL: &'static str = "https://api.github.com/";

/// Struct with client state, values in this struct may be modified at any time,
/// the new values will be used from the next API call
#[derive(Clone, Debug, PartialEq)]
pub struct Client {

    /// Base URL to the API, can be modified to use the enterprise API.
    pub api_url:    String,
    /// Value used for the User-Agent key in request headers.
    pub user_agent: String,
    /// Optional authorization token, acquired from https://github.com/settings/tokens,
    /// should be set to None when purely accessing endpoints requiring no authentication.
    pub auth_token: Option<String>,
}

impl Client {

    /// Creates a Client state with the default API URL.
    pub fn new(user: &str, token: Option<String>) -> Client {
        Client::with_url(DEFAULT_API_URL, user, token)
    }

    /// Creates a Client state an API URL other than the default
    pub fn with_url(url: &str, user: &str, token: Option<String>) -> Client {
        Client {
            api_url:    url.to_string(),
            user_agent: user.to_string(),
            auth_token: token
        }
    }
}

#[cfg(test)]
mod client_test {

    #[test]
    fn client_new() {
        let client = super::Client::new("rustyhub-test/0.0.0", Some("test-token".to_string()));
        assert!(client.api_url    == String::from("https://api.github.com/"));
        assert!(client.user_agent == String::from("rustyhub-test/0.0.0"));
        assert!(client.auth_token == Some(String::from("test-token")));
    }

    #[test]
    fn client_with_url() {
        let client = super::Client::with_url("https://api.github.com/", "rustyhub-test/0.0.0", Some("test-token".to_string()));
        assert!(client.api_url    == String::from("https://api.github.com/"));
        assert!(client.user_agent == String::from("rustyhub-test/0.0.0"));
        assert!(client.auth_token == Some(String::from("test-token")));
    }
}
