// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # References
//!
//! These are the responses and API call functions related
//! to the references endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/git/refs/

use serde_json;

use client::Client;
use error;
use utils;

/// Response type to references endpoints.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Reference {
    #[serde(rename = "ref")]
    pub _ref:   String,
    pub url:    String,
    pub object: Object
}

/// Sub-component of Reference.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Object {
    #[serde(rename = "type")]
    pub object_type: String,
    pub sha:         String,
    pub url:         String
}

/// Internal parameter
#[derive(Clone, Debug, PartialEq, Serialize)]
struct RefCreateParam {
    #[serde(rename = "ref")]
    _ref: String,
    sha:  String
}

/// Internal parameter
#[derive(Clone, Debug, PartialEq, Serialize)]
struct RefUpdateParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    sha:   String
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait ReferencesExt {


    /// \[[Reference](https://developer.github.com/v3/git/refs/#get-a-reference)\]
    /// Returns a reference.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/refs/:ref
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `_ref`: Name of the fully qualified reference.
    fn get_repos_owner_repo_git_refs_ref(&mut self, owner: String, repo: String, _ref: String) -> Result<Reference, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/refs/#get-all-references)\]
    /// Returns all references.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/refs
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_repos_owner_repo_git_refs(&mut self, owner: String, repo: String) -> Result<Vec<Reference>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/refs/#get-all-references)\]
    /// Returns all references in the heads namespace.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/refs/heads
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_repos_owner_repo_git_refs_heads(&mut self, owner: String, repo: String) -> Result<Vec<Reference>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/refs/#get-all-references)\]
    /// Returns all references in the tags namespace.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/refs/tags
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_repos_owner_repo_git_refs_tags(&mut self, owner: String, repo: String) -> Result<Vec<Reference>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/refs/#create-a-reference)\]
    /// Creates a reference.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/git/refs
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `_ref`: Name of the fully qualified reference.
    /// * `sha`: SHA1 value to set the reference to.
    fn post_repos_owner_repo_git_refs(&mut self, owner: String, repo: String, _ref: String, sha: String) -> Result<Reference, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/refs/#update-a-reference)\]
    /// Updates a reference.
    /// ## Endpoint:
    /// PATCH /repos/:owner/:repo/git/refs/:ref
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `_ref`: Name of the fully qualified reference.
    /// * `sha`: SHA1 value to set the reference to.
    /// * `force`: Default: false, Indicates whether to
    /// force the update or to make sure the update is a
    /// fast-forward update. Leaving this out or setting it
    /// to false will make sure you're not overwriting work.
    fn patch_repos_owner_repo_git_refs_ref(&mut self, owner: String, repo: String, _ref: String, sha: String, force: Option<bool>) -> Result<Reference, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/refs/#delete-a-reference)\]
    /// Deletes a reference.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/git/refs/:ref
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `_ref`: Name of the fully qualified reference.
    fn delete_repos_owner_repo_git_refs_ref(&mut self, owner: String, repo: String, _ref: String) -> Result<(), error::Error>;
}

impl ReferencesExt for Client {

    fn get_repos_owner_repo_git_refs_ref(&mut self, owner: String, repo: String, _ref: String) -> Result<Reference, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/git/refs/{}", owner, repo, _ref))
    }

    fn get_repos_owner_repo_git_refs(&mut self, owner: String, repo: String) -> Result<Vec<Reference>, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/git/refs", owner, repo))
    }

    fn get_repos_owner_repo_git_refs_heads(&mut self, owner: String, repo: String) -> Result<Vec<Reference>, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/git/refs/heads", owner, repo))
    }

    fn get_repos_owner_repo_git_refs_tags(&mut self, owner: String, repo: String) -> Result<Vec<Reference>, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/git/refs/tags", owner, repo))
    }

    fn post_repos_owner_repo_git_refs(&mut self, owner: String, repo: String, _ref: String, sha: String) -> Result<Reference, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&RefCreateParam{
            _ref: _ref,
            sha:  sha
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/git/refs", owner, repo), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn patch_repos_owner_repo_git_refs_ref(&mut self, owner: String, repo: String, _ref: String, sha: String, force: Option<bool>) -> Result<Reference, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&RefUpdateParam{
            sha:   sha,
            force: force
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.patch_body(format!("/repos/{}/{}/git/refs/{}", owner, repo, _ref), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn delete_repos_owner_repo_git_refs_ref(&mut self, owner: String, repo: String, _ref: String) -> Result<(), error::Error>{
        match self.delete(format!("/repos/{}/{}/git/refs/{}", owner, repo, _ref), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }
}


//TODO: TESTS
