// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Comments
//!
//! These are the responses and API call functions related
//! to the comments endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/issues/comments/

use hyper::{Error as HyperError, Url};
use serde_json;

use client::Client;
use common::Comment;
use issues::{Direction, Sort};
use error;
use utils;

/// Internal parameter
#[derive(Clone, Debug, PartialEq, Serialize)]
struct Body {
    body: String
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait CommentsExt {

    /// \[[Reference](https://developer.github.com/v3/issues/comments/#list-comments-on-an-issue)\]
    /// Returns the list of comments on an issue.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/issues/:number/comments
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Issue number.
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_repos_owner_repo_issues_number_comments(&mut self, owner: String, repo: String, number: u64, since: String) -> Result<Vec<Comment>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/comments/#list-comments-in-a-repository)\]
    /// Returns the list of comments in a repository.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/issues/comments
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `direction`: Ignored if None,
    /// the requested sorting direction of returned issues.
    /// * `sort`: Default: `Sort::Created`, the requested
    /// sorting of returned issues.
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_repos_owner_repo_issues_comments(&mut self, owner: String, repo: String, sort: Option<Sort>, direction: Option<Direction>, since: String) -> Result<Vec<Comment>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/comments/#get-a-single-comment)\]
    /// Returns a single comment.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/issues/comments/:id
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `id`: Comment ID.
    fn get_repos_owner_repo_issues_comments_id(&mut self, owner: String, repo: String, id: u64) -> Result<Comment, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/comments/#create-a-comment)\]
    /// Creates a comment.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/issues/:number/comments
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Issue number to put the comment on.
    /// * `body`: Content of the comment.
    fn post_repos_owner_repo_issues_number_comments(&mut self, owner: String, repo: String, number: u64, body: String) -> Result<Comment, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/comments/#edit-a-comment)\]
    /// Edits a comment.
    /// ## Endpoint:
    /// PATCH /repos/:owner/:repo/issues/:number/comments
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `id`: Comment ID.
    /// * `body`: Content of the comment.
    fn patch_repos_owner_repo_issues_comments_id(&mut self, owner: String, repo: String, id: u64, body: String) -> Result<Comment, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/comments/#edit-a-comment)\]
    /// Edits a comment.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/issues/comments/:id
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `id`: Comment ID.
    fn delete_repos_owner_repo_issues_comments_id(&mut self, owner: String, repo: String, id: u64) -> Result<(), error::Error>;
}

impl CommentsExt for Client {

    fn get_repos_owner_repo_issues_number_comments(&mut self, owner: String, repo: String, number: u64, since: String) -> Result<Vec<Comment>, error::Error> {

        let mut url = match Url::parse(&format!("{}/repos/{}/{}/issues/{}/comments", self.api_url, owner, repo, number)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/repos/{}/{}/issues/{}/comments", owner, repo, number))
    }

    fn get_repos_owner_repo_issues_comments(&mut self, owner: String, repo: String, sort: Option<Sort>, direction: Option<Direction>, since: String) -> Result<Vec<Comment>, error::Error> {

        let mut url = match Url::parse(&format!("{}/repos/{}/{}/issues/comments", self.api_url, owner, repo)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = sort {
                query_pairs.append_pair("sort", &param.to_str());
            }

            if let Some(param) = direction {
                query_pairs.append_pair("direction", &param.to_str());
            }

            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/repos/{}/{}/issues/comments", owner, repo))
    }

    fn get_repos_owner_repo_issues_comments_id(&mut self, owner: String, repo: String, id: u64) -> Result<Comment, error::Error> {
        utils::request_endpoint(self, format!("/repos/{}/{}/issues/comments/{}", owner, repo, id))
    }

    fn post_repos_owner_repo_issues_number_comments(&mut self, owner: String, repo: String, number: u64, body: String) -> Result<Comment, error::Error> {

        //Create body
        let body_data = try!(serde_json::to_string(&Body{
            body: body
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/issues/{}/comments", owner, repo, number), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn patch_repos_owner_repo_issues_comments_id(&mut self, owner: String, repo: String, id: u64, body: String) -> Result<Comment, error::Error> {

        let body_data = try!(serde_json::to_string(&Body{
            body: body
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.patch_body(format!("/repos/{}/{}/issues/comments/{}", owner, repo, id), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn delete_repos_owner_repo_issues_comments_id(&mut self, owner: String, repo: String, id: u64) -> Result<(), error::Error> {
        match self.delete(format!("/repos/{}/{}/issues/comments/{}", owner, repo, id), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }
}

//TODO: Handle custom media types
//TODO: TESTS
