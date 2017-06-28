// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Issues
//!
//! Modules for subsections of Gists in the Github API docs
//! and endpoints for Gists.
//!
//! Reference: https://developer.github.com/v3/gists/

use std::collections::BTreeMap;
use hyper::{Error as HyperError, Uri};
use serde_json;

use client::Client;
use common::User;
use error;
use utils;

/// Return type for Gist endpoints
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Gist {
    pub url:          String,
    pub forks_url:    String,
    pub commits_url:  String,
    pub id:           String,
    pub description:  String,
    pub public:       bool,
    pub owner:        User,
    //pub user: null, Always seem to be null
    pub files:        BTreeMap<String, File>,
    pub truncated:    bool,
    pub comments:     u64,
    pub comments_url: String,
    pub html_url:     String,
    pub git_pull_url: String,
    pub git_push_url: String,
    pub created_at:   String,
    pub updated_at:   String,
    pub forks:        Option<Vec<Fork>>,
    pub history:      Option<Vec<Commit>>
}

/// Sub-component of the Gist response
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct File {
    pub size:      u64,
    pub raw_url:   String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub truncated: bool,
    pub language:  String,
}

/// Sub-component of the Gist response
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Fork {
    pub user:       User,
    pub url:        String,
    pub id:         String,
    pub created_at: String,
    pub updated_at: String
}

/// Sub-component of the Gist response
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Commit {
    pub url:           String,
    pub version:       String,
    pub user:          User,
    pub change_status: ChangeStatus,
    pub committed_at:  String
}

/// Sub-component of the History
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ChangeStatus {
    pub deletions: u64,
    pub additions: u64,
    pub total:     u64
}

//TODO: More precise doc + example
/// Input parameters for Gist creation and editing
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GistParam {
    pub description: String,
    pub public: bool,
    pub files: BTreeMap<String, Option<FileContents>>,
}

///Sub-component of GistParam
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FileContents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub content: String
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait GistsExt {

    /// \[[Reference](https://developer.github.com/v3/gists/#list-a-users-gists)\]
    /// Returns the list of public gists for a user.
    /// ## Endpoint:
    /// GET /users/:username/gists
    /// ## Parameters
    /// * `username`: Name fo the user.
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_users_username_gists(&mut self, username: String, since: String) -> Result<Vec<Gist>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#list-a-users-gists)\]
    /// Returns the list of the authenticated user's gists
    /// or if called anonymously, returns all public gists.
    /// ## Endpoint:
    /// GET /gists
    /// ## Parameters
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_gists(&mut self, since: String) -> Result<Vec<Gist>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#list-all-public-gists)\]
    /// Returns the list of all public gists sorted by most
    /// recently updated to least recently updated.
    /// ## Endpoint:
    /// GET /gists/public
    /// ## Parameters
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_gists_public(&mut self, since: String) -> Result<Vec<Gist>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#list-starred-gists)\]
    /// Returns the list of the authenticated user's
    /// starred gists.
    /// ## Endpoint:
    /// GET /gists/starred
    /// ## Parameters
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_gists_starred(&mut self, since: String) -> Result<Vec<Gist>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#get-a-single-gist)\]
    /// Get a single gist.
    /// ## Endpoint:
    /// GET /gists/:id
    /// ## Parameters
    /// * `id`: Gist ID.
    fn get_gists_id(&mut self, id: u64) -> Result<Gist, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#get-a-specific-revision-of-a-gist)\]
    /// Get a specific revision of a gist.
    /// ## Endpoint:
    /// GET /gists/:id/:sha
    /// ## Parameters
    /// * `id`: Gist ID.
    /// * `sha`: Gist SHA hash.
    fn get_gists_id_sha(&mut self, id: u64, sha: String) -> Result<Gist, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#create-a-gist)\]
    /// Create a gist.
    /// ## Endpoint:
    /// POST /gists
    /// ## Parameters
    /// * `gist`: Input parameters, see Reference and
    /// GistParam.
    fn post_gists(&mut self, gist: GistParam) -> Result<Gist, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#edit-a-gist)\]
    /// Edit a gist
    /// ## Endpoint:
    /// PATCH /gists/:id
    /// ## Parameters
    /// * `id`: ID of the gist to be edited.
    /// * `gist`: Input parameters, see Reference and
    /// GistParam.
    fn patch_gists_id(&mut self, id: u64, gist: GistParam) -> Result<Gist, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#list-gist-commits)\]
    /// List gist commits
    /// ## Endpoint:
    /// GET /gists/:id/commits
    /// ## Parameters
    /// * `id`: ID of the gist.
    fn get_gists_id_commits(&mut self, id: u64) -> Result<Vec<Commit>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#star-a-gist)\]
    /// Star a gist
    /// ## Endpoint:
    /// PUT /gists/:id/star
    /// ## Parameters
    /// * `id`: ID of the gist to be starred.
    fn put_gists_id_star(&mut self, id: u64) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#unstar-a-gist)\]
    /// Unstar a gist
    /// ## Endpoint:
    /// DELETE /gists/:id/star
    /// ## Parameters
    /// * `id`: ID of the gist to be unstarred.
    fn delete_gists_id_star(&mut self, id: u64) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#check-if-a-gist-is-starred)\]
    /// Check if a gist is starred
    /// ## Endpoint:
    /// GET /gists/:id/star
    /// ## Parameters
    /// * `id`: ID of the gist to be checked.
    fn get_gists_id_star(&mut self, id: u64) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#fork-a-gist)\]
    /// Fork a gist
    /// ## Endpoint:
    /// POST /gists/:id/forks
    /// ## Parameters
    /// * `id`: ID of the gist to be forked.
    fn post_gists_id_forks(&mut self, id: u64) -> Result<Vec<Commit>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#list-gist-forks)\]
    /// List gist forks
    /// ## Endpoint:
    /// GET /gists/:id/forks
    /// ## Parameters
    /// * `id`: ID of the gist.
    fn get_gists_id_forks(&mut self, id: u64) -> Result<Vec<Fork>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/#delete-a-gist)\]
    /// Delete a gist
    /// ## Endpoint:
    /// DELETE /gists/:id
    /// ## Parameters
    /// * `id`: ID of the gist to be deleted.
    fn delete_gists_id(&mut self, id: u64) -> Result<(), error::Error>;
}

impl GistsExt for Client {

    fn get_users_username_gists(&mut self, username: String, since: String) -> Result<Vec<Gist>, error::Error>{

        let mut url = match Uri::to_str(&format!("{}/users/{}/gists", self.api_url, username)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/users/{}/gists?{}", username, url.query().unwrap()))
    }

    fn get_gists(&mut self, since: String) -> Result<Vec<Gist>, error::Error>{

        let mut url = match Uri::to_str(&format!("{}/gists", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/gists?{}", url.query().unwrap()))
    }

    fn get_gists_public(&mut self, since: String) -> Result<Vec<Gist>, error::Error>{

        let mut url = match Uri::to_str(&format!("{}/gists/public", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/gists/public?{}", url.query().unwrap()))
    }

    fn get_gists_starred(&mut self, since: String) -> Result<Vec<Gist>, error::Error>{

        let mut url = match Uri::to_str(&format!("{}/gists/starred", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/gists/starred?{}", url.query().unwrap()))
    }

    fn get_gists_id(&mut self, id: u64) -> Result<Gist, error::Error>{
        utils::request_endpoint(self, format!("/gists/{}", id))
    }

    fn get_gists_id_sha(&mut self, id: u64, sha: String) -> Result<Gist, error::Error>{
        utils::request_endpoint(self, format!("/gists/{}/{}", id, sha))
    }

    fn post_gists(&mut self, gist: GistParam) -> Result<Gist, error::Error>{

        //Serialize body
        let gist_body = try!(serde_json::to_string(&gist).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body("/gists".to_string(), None, gist_body));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn patch_gists_id(&mut self, id: u64, gist: GistParam) -> Result<Gist, error::Error>{

        //Serialize body
        let gist_body = try!(serde_json::to_string(&gist).map_err(error::Error::Parsing));

        let mut response     = try!(self.patch_body(format!("/gists/{}", id), None, gist_body));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn get_gists_id_commits(&mut self, id: u64) -> Result<Vec<Commit>, error::Error>{
        utils::request_endpoint(self, format!("/gists/{}/commit", id))
    }

    fn put_gists_id_star(&mut self, id: u64) -> Result<(), error::Error>{
        match self.put(format!("/gists/{}/star", id), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn delete_gists_id_star(&mut self, id: u64) -> Result<(), error::Error>{
        match self.delete(format!("/gists/{}/star", id), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn get_gists_id_star(&mut self, id: u64) -> Result<(), error::Error>{
        match self.get(format!("/gists/{}/star", id), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn post_gists_id_forks(&mut self, id: u64) -> Result<Vec<Commit>, error::Error>{
        let mut response     = try!(self.post(format!("/gists/{}/forks", id), None));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn get_gists_id_forks(&mut self, id: u64) -> Result<Vec<Fork>, error::Error>{
        utils::request_endpoint(self, format!("/gists/{}/forks", id))
    }

    fn delete_gists_id(&mut self, id: u64) -> Result<(), error::Error>{
        match self.delete(format!("/gists/{}", id), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }
}

//TODO: Custom Media Types
//TODO: TESTS
