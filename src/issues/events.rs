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
//! Reference: https://developer.github.com/v3/issues/events/

use client::Client;
use common::User;
use issues::Issue;
use error;
use utils;


/// Response element to the list events for an issue
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct IssueEvent {
    pub id:         u64,
    pub url:        String,
    pub actor:      User,
    pub event:      String,
    pub commit_id:  String,
    pub commit_url: String,
    pub created_at: String
}

/// Response element to the list events for a repository
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct IssueEventRepo {
    pub id: u64,
    pub url: String,
    pub actor: User,
    pub event:      String,
    pub commit_id:  String,
    pub commit_url: String,
    pub created_at: String,
    pub issue:      Issue
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait IssueEventsExt {

    /// \[[Reference](https://developer.github.com/v3/issues/events/#list-events-for-an-issue)\]
    /// Returns the list of events for an issue.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/issues/:issue_number/events
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `issue_number`: Number of the issue.
    fn get_owner_repo_issues_issue_number_events(&mut self, owner: String, repo: String, issue_number: u64) -> Result<Vec<IssueEvent>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/events/#list-events-for-a-repository)\]
    /// Returns the list of events for arepository.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/issues/events
    /// ## Parameters
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_owner_repo_issues_events(&mut self, owner: String, repo: String) -> Result<Vec<IssueEventRepo>, error::Error>;
}

impl IssueEventsExt for Client {

    fn get_owner_repo_issues_issue_number_events(&mut self, owner: String, repo: String, issue_number: u64) -> Result<Vec<IssueEvent>, error::Error> {
        utils::request_endpoint(self, format!("/repos/{}/{}/issues/{}/events", owner, repo, issue_number))
    }

    fn get_owner_repo_issues_events(&mut self, owner: String, repo: String) -> Result<Vec<IssueEventRepo>, error::Error> {
        utils::request_endpoint(self, format!("/repos/{}/{}/issues/events", owner, repo))
    }
}


//TODO: TESTS
