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

/// Structure representing a commit
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Commit {
    /// SHA of the commit.
    pub sha:       String,
    /// URL of the commit.
    pub url:       String,
    /// Author of the commit.
    pub author:    CommitUser,
    /// Committer for this commit.
    pub committer: CommitUser,
    /// Commit message.
    pub message:   String,
    /// Tree object.
    pub tree:      CommitParent,
    /// Parent commit.
    pub parents:   Vec<CommitParent>
}

/// Sub-component of Commit
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommitUser {
    /// Date at which the commit was authored or committed,
    /// a timestamp in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`)
    /// format.
    pub date:  String,
    /// Name fo author/committer.
    pub name:  String,
    /// Email address of author/commiter
    pub email: String
}

/// Sub-component of Commit
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CommitParent {
    /// URL of the object.
    url: String,
    /// SHA of the object.
    sha: String
}

/// Parameters for creating a commit
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitParam {
    /// Commit message.
    message:  String,
    /// The SHA of the tree object this commit points to.
    tree:     String,
    /// Array of the SHAs of the commits' parents.
    parents:  Vec<String>,
    /// Optional author parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    author:   Option<CommitUser>,
    /// Optional committer parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    committer: Option<CommitUser>
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait CommitsExt {

    /// \[[Reference](https://developer.github.com/v3/git/commits/#get-a-commit)\]
    /// Returns a commit.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/commits/:sha
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `sha`: SHA of the commit.
    fn get_repos_owner_repo_git_commits_sha(&mut self, owner: String, repo: String, sha: String) -> Result<Commit, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/commits/#create-a-commit)\]
    /// Creates a commit.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/blobs/:sha
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `commit`: Parameters for the commit.
    fn post_repos_owner_repo_git_commits(&mut self, owner: String, repo: String, commit: CommitParam) -> Result<Commit, error::Error>;
}

impl CommitsExt for Client {

    fn get_repos_owner_repo_git_commits_sha(&mut self, owner: String, repo: String, sha: String) -> Result<Commit, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/git/commits/{}", owner, repo, sha))
    }

    fn post_repos_owner_repo_git_commits(&mut self, owner: String, repo: String, commit: CommitParam) -> Result<Commit, error::Error>{
        //Create body
        let body_data = try!(serde_json::to_string(&commit).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/git/commits", owner, repo), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)

    }
}

//TODO: Commit signature verification once out of preview
//TODO: TESTS
