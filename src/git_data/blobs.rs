// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Blobs
//!
//! These are the responses and API call functions related
//! to the blobs endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/git/blobs/

use serde_json;

use client::Client;
use error;
use utils;

/// Response to get a blob.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Blob {
    content:  String,
    encoding: String,
    url:      String,
    sha:      String,
    size:     u64
}

/// Response to create a blob.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BlobCreate {
    url:      String,
    sha:      String,
}

/// Internal Parameter
#[derive(Clone, Debug, PartialEq, Serialize)]
struct BlobParam {
    content:  String,
    encoding: String,
}

/// Encoding types for content when creating a blob.
pub enum BlobEncoding {
    Base64,
    UTF8
}

impl BlobEncoding {
    fn to_str(self) -> String {
        match self {
            BlobEncoding::Base64 => "base64".to_string(),
            BlobEncoding::UTF8   => "utf-8".to_string(),
        }
    }
}


////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait BlobExt {

    /// \[[Reference](https://developer.github.com/v3/git/blobs/#get-a-blob)\]
    /// Returns a blob.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/blobs/:sha
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `sha`: SHA of the blob.
    fn get_repos_owner_repo_git_blobs_sha(&mut self, owner: String, repo: String, sha: String) -> Result<Blob, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/blobs/#create-a-blob)\]
    /// Creates a blob.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/git/blobs
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `content`: Desired content of the blob.
    /// * `encoding`: Encoding of the content.
    fn post_repos_owner_repo_git_blobs(&mut self, owner: String, repo: String, content: String, encoding: BlobEncoding) -> Result<BlobCreate, error::Error>;
}

impl BlobExt for Client {

    fn get_repos_owner_repo_git_blobs_sha(&mut self, owner: String, repo: String, sha: String) -> Result<Blob, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/git/blobs/{}", owner, repo, sha))
    }

    fn post_repos_owner_repo_git_blobs(&mut self, owner: String, repo: String, content: String, encoding: BlobEncoding) -> Result<BlobCreate, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&BlobParam{
            content:  content,
            encoding: encoding.to_str()
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/git/blobs", owner, repo), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }
}

//TODO: Handle custom media types
//TODO: TESTS
