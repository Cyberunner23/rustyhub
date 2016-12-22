
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

use serde_json;

use hyper::client::{Client as HyperClient, Response};
//use hyper::error::Error as HyperError;
use hyper::header::{Accept,
                    Authorization,
                    Bearer,
                    Headers,
                    Location,
                    qitem,
                    UserAgent};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;

use error;

/// The default API URL.
static DEFAULT_API_URL: &'static str = "https://api.github.com";

/// Struct with client state, values in this struct may be modified at any time,
/// the new values will be used from the next API call
#[derive(Debug)]
pub struct Client {

    pub http_client: HyperClient,
    /// Base URL to the API, can be modified to use the enterprise API.
    pub api_url:     String,
    /// Value used for the User-Agent key in request headers.
    pub user_agent:  String,
    /// Optional authorization token, acquired from https://github.com/settings/tokens,
    /// should be set to None when purely accessing endpoints requiring no authentication.
    pub auth_token:  Option<String>,
}

impl Client {

    /// Creates a Client state with the default API URL.
    pub fn new(user_agent: &str, token: Option<String>) -> Client {
        Client::with_url(DEFAULT_API_URL, user_agent, token)
    }

    /// Creates a Client state an API URL other than the default
    pub fn with_url(url: &str, user_agent: &str, token: Option<String>) -> Client {
        Client {
            http_client: HyperClient::new(),
            api_url:     url.to_string(),
            user_agent:  user_agent.to_string(),
            auth_token:  token
        }
    }

    /// Returns the default header used
    pub fn get_default_header(&mut self) -> Headers {
        let mut header = Headers::new();
        header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3+json".to_string()), vec![]))])); //"application/vnd.github.v3+json"
        header.set(UserAgent(self.user_agent[..].to_owned()));
        if let Some(ref auth) = self.auth_token {header.set(Authorization(Bearer{token: auth[..].to_owned()}))}
        return header;
    }

    //Utils
    fn get_redirect(source_url: &String, response: &Response) -> Option<String> {

        match response.status {
            StatusCode::MovedPermanently  |
            StatusCode::TemporaryRedirect |
            StatusCode::PermanentRedirect => match response.headers.get() {
                Some(&Location(ref loc)) => {
                    if response.status == StatusCode::PermanentRedirect {
                        info!("{} as been permanently redirected, please notify the rustyhub developer that it has been moved to {}", source_url, loc);
                    }
                    Some(loc.clone())
                },
                None => None
            },
            _ => None
        }
    }

    fn get_error(response: &mut Response) -> Option<error::Error> {
        match response.status {
            StatusCode::BadRequest  |
            StatusCode::UnprocessableEntity => {
                let mut body_data = String::new();
                match response.read_to_string(&mut body_data) {
                    Ok(_)    => (),
                    Err(err) => return Some(error::Error::STDIO(err))
                }

                match serde_json::from_str(&body_data) {
                    Ok(err_response) => return Some(error::Error::Github(err_response)),
                    Err(err)         => return Some(error::Error::Parsing(err))
                }
            },
            _ => None
        }
    }

    //HTTP Methods
    pub fn get(&mut self, endpoint: &'static str, header: Option<Headers>) -> Result<Response, error::Error> {

        //if no headers use default
        let request_header = match header {
            Some(header) => header,
            None         => self.get_default_header()
        };

        //In case we get redirected, we will need the same headers
        let     request_header_copy = request_header.clone();
        let mut response = match self.http_client.get(&format!("{}{}", self.api_url, endpoint)[..]).headers(request_header).send() {
            Ok(response) => response,
            Err(err)     => return Err(error::Error::HTTP(err))
        };

        //Handle redirects
        while let Some(loc) = Client::get_redirect(&format!("{}{}", self.api_url, endpoint), &response) {
            response = match self.http_client.get(&loc[..]).headers(request_header_copy.clone()).send() {
                Ok(response) => response,
                Err(err)     => return Err(error::Error::HTTP(err))
            };
        }

        //Handle error
        if let Some(err) = Client::get_error(&mut response) {
            return Err(err)
        }

        Ok(response)
    }
}



//TODO: Tests

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
