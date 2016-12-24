
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

use serde_json;

use hyper::client::{Client as HyperClient, Body, Response};
//use hyper::error::Error as HyperError;
use hyper::header::{Accept,
                    Authorization,
                    Bearer,
                    Headers,
                    Location,
                    qitem,
                    UserAgent};
use hyper::method::Method;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;

use error;

/// The default API URL.
static DEFAULT_API_URL: &'static str = "https://api.github.com";

/// Struct with client state, values in this struct may be modified at any time,
/// the new values will be used from the next API call
/// Handles averything related to HTTP
#[derive(Debug)]
pub struct Client {

    /// Internal http client.
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

    /// Creates a Client state an API URL other than the default.
    pub fn with_url(url: &str, user_agent: &str, token: Option<String>) -> Client {
        Client {
            http_client: HyperClient::new(),
            api_url:     url.to_string(),
            user_agent:  user_agent.to_string(),
            auth_token:  token
        }
    }

    /// Returns the default headers used throughout the crate.
    pub fn get_default_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3+json".to_string()), vec![]))])); //"application/vnd.github.v3+json"
        headers.set(UserAgent(self.user_agent[..].to_owned()));
        if let Some(ref auth) = self.auth_token {headers.set(Authorization(Bearer{token: auth[..].to_owned()}))}
        return headers;
    }

    //Utils
    fn get_redirect(source_url: &String, response: &Response) -> Option<String> {
        match response.status {
            StatusCode::MovedPermanently  |
            StatusCode::TemporaryRedirect |
            StatusCode::PermanentRedirect =>
                response.headers.get().map(|&Location(ref loc)| {
                    if response.status == StatusCode::PermanentRedirect {
                        info!("{} as been permanently redirected, please notify
                              the rustyhub developer that it has been moved to {}", source_url, loc);
                    }
                    loc.clone()
                }),
            _ => None
        }
    }

    fn get_error(response: &mut Response) -> Option<error::Error> {
        match response.status {
            StatusCode::BadRequest  |
            StatusCode::UnprocessableEntity |
            StatusCode::NotFound => {
                let body_data = match Client::response_to_string(response) {
                    Ok(data) => data,
                    Err(err) => return Some(err)
                };

                match serde_json::from_str(&body_data) {
                    Ok(err_response) => return Some(error::Error::Github(err_response)),
                    Err(err)         => return Some(error::Error::Parsing(err))
                }
            },
            _ => None
        }
    }

    ///Utility to extract the body of an API response.
    pub fn response_to_string(response: &mut Response) -> Result<String, error::Error> {
        let mut body_data = String::new();
        response.read_to_string(&mut body_data)
                .map(|_| body_data)
                .map_err(error::Error::STDIO)
    }

    //option, none if not found
    /*pub fn extract_header_string(header: &Headers, field: &str) -> Result<Option<String>, error::Error> {

        match header.get_raw(field) {
            Some(value) => {
                match String::from_utf8(value[0].clone()) {
                    Ok(value_string) => return Ok(Some(value_string)),
                    Err(err)         => return Err(error::Error::STDUtf8(err))
                }
            }
            None => return Ok(None)
        }
    }

    pub fn extract_header_u64(header: &Headers, field: &str) -> Result<Option<u64>, error::Error> {
        let value = try!(Client::extract_header_string(&header, &field));
        match value {
            Some(value) => {
                match value.parse::<u64>() {
                    Ok(value_string) => return Ok(Some(value_string)),
                    Err(err)         => return Err(error::Error::STDParseInt(err))
                }
            },
            None => Ok(None)
        }
    }*/


    fn make_request(&self, method: Method, endpoint: String, headers: Option<Headers>) -> Result<Response, error::Error> {
        //if no headers use default
        let request_header = headers.unwrap_or_else(|| self.get_default_headers());

        //In case we get redirected, we will need the same headers
        let     request_header_copy = request_header.clone();
        let mut response = try!(self.http_client.request(method, &format!("{}{}", self.api_url, endpoint)[..])
            .headers(request_header)
            .send()
            .map_err(error::Error::HTTP));

        //Handle redirects
        while let Some(loc) = Client::get_redirect(&format!("{}{}", self.api_url, endpoint), &mut response) {
            response = try!(self.http_client.get(&loc[..])
                .headers(request_header_copy.clone())
                .send()
                .map_err(error::Error::HTTP));
        }

        //Handle error
        if let Some(err) = Client::get_error(&mut response) {
            return Err(err)
        }

        Ok(response)
    }

    fn make_request_body(&self,
                         method: Method,
                         endpoint: String,
                         headers: Option<Headers>,
                         body: String) -> Result<Response, error::Error> {
        //if no headers use default
        let request_header = headers.unwrap_or_else(|| self.get_default_headers());

        //In case we get redirected, we will need the same headers
        let     request_header_copy = request_header.clone();
        let     body_len  = body.clone().len();
        let     body_copy = body.clone();
        let mut response  = try!(self.http_client.request(method.clone(), &format!("{}{}", self.api_url, endpoint)[..])
            .headers(request_header)
            .body(Body::BufBody(&body.into_bytes()[..], body_len))
            .send()
            .map_err(error::Error::HTTP));

        //Handle redirects
        while let Some(loc) = Client::get_redirect(&format!("{}{}", self.api_url, endpoint), &mut response) {
            response = try!(self.http_client.request(method.clone(), &loc[..])
                .headers(request_header_copy.clone())
                .body(Body::BufBody(&body_copy.clone().into_bytes()[..], body_len))
                .send()
                .map_err(error::Error::HTTP));
        }

        //Handle error
        if let Some(err) = Client::get_error(&mut response) {
            return Err(err)
        }

        Ok(response)
    }


    //HTTP Methods
    ///GET request with optionally overridden headers.
    pub fn get(&self, endpoint: String, headers: Option<Headers>) -> Result<Response, error::Error> {
        self.make_request(Method::Get, endpoint, headers)
    }

    ///POST request with optionally overridden headers.
    pub fn post(&self, endpoint: String, headers: Option<Headers>) -> Result<Response, error::Error> {
        self.make_request(Method::Post, endpoint, headers)
    }

    ///PUT request with optionally overridden headers.
    pub fn put(&self, endpoint: String, headers: Option<Headers>) -> Result<Response, error::Error> {
        self.make_request(Method::Put, endpoint, headers)
    }

    ///PATCH request with optionally overridden headers.
    pub fn patch(&self, endpoint: String, headers: Option<Headers>) -> Result<Response, error::Error> {
        self.make_request(Method::Patch, endpoint, headers)
    }

    ///DELETE request with optionally overridden headers.
    pub fn delete(&self, endpoint: String, headers: Option<Headers>) -> Result<Response, error::Error> {
        self.make_request(Method::Delete, endpoint, headers)
    }


    ///GET request with optionally overridden headers and a body.
    pub fn get_body(&self, endpoint: String, headers: Option<Headers>, body: String) -> Result<Response, error::Error> {
        self.make_request_body(Method::Get, endpoint, headers, body)
    }

    ///POST request with optionally overridden headers and a body.
    pub fn post_body(&self, endpoint: String, headers: Option<Headers>, body: String) -> Result<Response, error::Error> {
        self.make_request_body(Method::Post, endpoint, headers, body)
    }

    ///PUT request with optionally overridden headers and a body.
    pub fn put_body(&self, endpoint: String, headers: Option<Headers>, body: String) -> Result<Response, error::Error> {
        self.make_request_body(Method::Put, endpoint, headers, body)
    }

    ///PATCH request with optionally overridden headers and a body.
    pub fn patch_body(&self, endpoint: String, headers: Option<Headers>, body: String) -> Result<Response, error::Error> {
        self.make_request_body(Method::Patch, endpoint, headers, body)
    }

    ///DELETE request with optionally overridden headers and a body.
    pub fn delete_body(&self, endpoint: String, headers: Option<Headers>, body: String) -> Result<Response, error::Error> {
        self.make_request_body(Method::Delete, endpoint, headers, body)
    }
}


//TODO: Tests

#[cfg(test)]
mod client_test {

    #[test]
    fn client_new() {
        let client = super::Client::new("rustyhub-test/0.0.0", Some("test-token".to_string()));
        assert!(client.api_url    == String::from("https://api.github.com"));
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
