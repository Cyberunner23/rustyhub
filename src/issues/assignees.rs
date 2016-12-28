// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Events
//!
//! These are the responses and API call functions related
//! to the event endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/issues/assignees/

use client::Client;
use common::User;
use issues::Issue;
use error;
use utils;

///Internal parameter
struct Assignees {
    assignees: Vec<String>
}

pub trait AssigneesExt {

    /// \[[Reference](https://developer.github.com/v3/issues/assignees/#list-assignees)\]
    /// Returns the list of all available assignees to
    /// which issues may be assigned.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/assignees
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_repos_owner_repo_assignees(&mut self, owner: String, repo: String) -> Result<Vec<User>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/assignees/#check-assignee)\]
    /// Checks is a user is an assignee for the repository.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/assignees/:assignee
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `assignee`: Username to be checked.
    fn get_repos_owner_repo_assignees_assignee(&mut self, owner: String, repo: String, assignee: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/assignees/#add-assignees-to-an-issue)\]
    /// Adds assignees to the issue.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/issues/:number/assignees
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `number`: Issue number to add assignees.
    /// * `assignees`: Vec of Usernames to add as assignees.
    fn post_repos_owner_repo_issues_number_assignees(&mut self, owner: String, repo: String, number: u64, assignees: Vec<String>) -> Result<Issue, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/assignees/#remove-assignees-from-an-issue)\]
    /// Deletes assignees from the issue.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/issues/:number/assignees
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `number`: Issue number to add assignees.
    /// * `assignees`: Vec of Usernames to add as assignees.
    fn delete_repos_owner_repo_issues_number_assignees(&mut self, owner: String, repo: String, number: u64, assignees: Vec<String>) -> Result<Issue, error::Error>;
}

impl AssigneesExt for Client {

    fn get_repos_owner_repo_assignees(&mut self, owner: String, repo: String) -> Result<Vec<User>, error::Error> {
        utils::request_endpoint(self, format!("/repos/{}/{}/assignees", owner, repo))
    }

    fn get_repos_owner_repo_assignees_assignee(&mut self, owner: String, repo: String, assignee: String) -> Result<(), error::Error> {
        match self.put(format!("/repos/{}/{}/assignees/{}", owner, repo, assignee), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn post_repos_owner_repo_issues_number_assignees(&mut self, owner: String, repo: String, number: u64, assignees: Vec<String>) -> Result<Issue, error::Error> {

        let assignees_body = try!(serde_json::to_string(&Assignees{
            assignees: assignees
            }).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/notifications/threads/{}/subscription?{}", id, url.query().unwrap()), None, assignees_body));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn delete_repos_owner_repo_issues_number_assignees(&mut self, owner: String, repo: String, number: u64, assignees: Vec<String>) -> Result<Issue, error::Error> {
        let assignees_body = try!(serde_json::to_string(&Assignees{
            assignees: assignees
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.delete_body(format!("/notifications/threads/{}/subscription?{}", id, url.query().unwrap()), None, assignees_body));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

}

//TODO: TESTS
