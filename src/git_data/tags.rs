// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Commits
//!
//! These are the responses and API call functions related
//! to the commits endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/git/commits/

use serde_json;

use client::Client;
use error;
use utils;

/// Return type of tags endpoints.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Tag {
    pub tag:     String,
    pub sha:     String,
    pub url:     String,
    pub message: String,
    pub tagger:  Tagger,
    pub object:  TagObject
}

/// Sub-component of Tag.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Tagger {
    /// Name of the person creating/created the tag.
    pub name:  String,
    /// Email of the person creating/created the tag.
    pub email: String,
    /// Date of when the object was tagged in
    /// ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    pub date:  String
}

/// Sub-component of Tag.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TagObject {
    #[serde(rename = "type")]
    pub object_type: String,
    pub sha:         String,
    pub url:         String
}

/// Parameters for Tag creation.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagParam {
    /// Tag name.
    pub tag: String,
    /// Tag message.
    pub message: String,
    /// SHA of object this is tagging
    pub object: String,
    /// commit, tag or blob
    #[serde(rename = "type")]
    pub tag_type: String,
    /// Info about person creating the tag.
    pub tagger: Tagger
}


////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait TagsExt {

    /// \[[Reference](https://developer.github.com/v3/git/tags/#get-a-tag)\]
    /// Returns a tag.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/tags/:sha
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `sha`: SHA of the tag.
    fn get_repos_owner_repo_git_tags_sha(&mut self, owner: String, repo: String, sha: String) -> Result<Tag, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/tags/#create-a-tag-object)\]
    /// Creates a tag.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/git/tags
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `tag`: Parameters for tag creation.
    fn post_repos_owner_repo_git_tags(&mut self, owner: String, repo: String, tag: TagParam) -> Result<Tag, error::Error>;
}

impl TagsExt for Client {

    fn get_repos_owner_repo_git_tags_sha(&mut self, owner: String, repo: String, sha: String) -> Result<Tag, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/git/tags/{}", owner, repo, sha))
    }

    fn post_repos_owner_repo_git_tags(&mut self, owner: String, repo: String, tag: TagParam) -> Result<Tag, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&tag).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/git/tags", owner, repo), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }
}

//TODO: Tag signature verification once out of preview.
//TODO: TESTS
