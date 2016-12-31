// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Trees
//!
//! These are the responses and API call functions related
//! to the trees endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/git/trees/

use serde_json;

use client::Client;
use error;
use utils;

/// Response to trees endpoints.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Tree {
    pub sha:       String,
    pub url:       String,
    pub tree:      Vec<TreeElem>,
    pub truncated: bool
}

/// Sub-component to Tree and TreeParam
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TreeElem {
    /// File referenced in the tree.
    pub path:      String,
    /// The file mode, see reference.
    pub mode:      String,
    /// blob, tree or commit
    #[serde(rename = "type")]
    pub tree_type: String,
    /// Size of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size:      Option<u64>,
    /// SHA1 checksum of the object in the tree
    pub sha:       String,
    /// URL to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url:       Option<String>,
    /// Content of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content:   Option<String>
}

/// Parameters for creating trees
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TreeParam {
    base_tree: String,
    tree:      Vec<TreeElem>
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait TreesExt {

    /// \[[Reference](https://developer.github.com/v3/git/trees/#get-a-tree)\]
    /// Returns a tree.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/trees/:sha
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `sha`: SHA of the tree.
    fn get_repos_owner_repo_git_trees_sha(&mut self, owner: String, repo: String, sha: String) -> Result<Tree, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/trees/#get-a-tree-recursively)\]
    /// Returns a tree recursively.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/git/trees/:sha?recursive=1
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `sha`: SHA of the tree.
    fn get_repos_owner_repo_git_trees_sha_recursive(&mut self, owner: String, repo: String, sha: String) -> Result<Tree, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/git/trees/#create-a-tree)\]
    /// Creates a tree.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/git/trees
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `tree`: Parameters for tree creation.
    fn post_repos_owner_repo_git_trees(&mut self, owner: String, repo: String, tree: TreeParam) -> Result<Tree, error::Error>;
}

impl TreesExt for Client {

    fn get_repos_owner_repo_git_trees_sha(&mut self, owner: String, repo: String, sha: String) -> Result<Tree, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/git/trees/{}", owner, repo, sha))
    }

    fn get_repos_owner_repo_git_trees_sha_recursive(&mut self, owner: String, repo: String, sha: String) -> Result<Tree, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/git/trees/{}?recursive=1", owner, repo, sha))
    }

    fn post_repos_owner_repo_git_trees(&mut self, owner: String, repo: String, tree: TreeParam) -> Result<Tree, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&tree).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/git/trees", owner, repo), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }
}

//TODO: TESTS
