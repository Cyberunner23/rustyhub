// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO: Add reactions once out of preview

//! # Issues
//!
//! Modules for subsections of Issues in the Github API docs
//! and endpoints for Issues.
//!
//! Reference: https://developer.github.com/v3/issues/

use hyper::{Error as HyperError, Url};
use serde_json;

use client::Client;
use common::{Repository, User};
use error;
use utils;

/// Endpoints for issue events.
pub mod events;
/// Endpoints for issue comments.
pub mod comments;


/// Return type for Issue endpoints and sub component for
/// Issue Event responses
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Issue {
    pub id:             u64,
    pub url:            String,
    pub repository_url: String,
    pub labels_url:     String,
    pub comments_url:   String,
    pub events_url:     String,
    pub html_url:       String,
    pub number:         u64,
    pub state:          String,
    pub title:          String,
    pub body:           String,
    pub user:           User,
    pub labels:         Vec<Label>,
    pub assignee:       User,
    pub milestone:      Milestone,
    pub locked:         bool,
    pub comments:       u64,
    pub pull_request:   Option<PullRequest>,
    pub closed_at:      Option<String>,
    pub created_at:     String,
    pub updated_at:     String,
    pub repository:     Option<Repository>,
    pub closed_by:      Option<User>,
    pub assignees:      Option<Vec<User>>
}

/// Parameters for the creation of an issue.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct IssueCreate {
    /// Title of the issue
    pub title:     String,
    /// Content of the issue
    pub body:      String,
    /// Usernames of the users that should be assigned to
    /// the issue. NOTE: Only users with push access can
    /// set the assignee for new issues. The assignee is
    /// silently dropped otherwise.
    pub assignees: Vec<User>,
    /// Number of the milestone to be associated with the
    /// issue. NOTE: Only users with push access can set
    /// labels for new issues. Labels are silently dropped
    /// otherwise.
    pub milestone: Option<u64>,
    /// Labels to be associated to the issue. NOTE: Only
    /// users with push access can set labels for new
    /// issues. Labels are silently dropped otherwise.
    pub labels:    Vec<String>
}

/// Parameters for the editing of an issue.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct IssueEdit {
    /// Title of the issue
    pub title:     String,
    /// Content of the issue
    pub body:      String,
    /// Usernames of the users that should be assigned to
    /// the issue. Set the Vec to empty to remove all
    /// assignees. NOTE: Only users with push access can
    /// set assignees for new issues. Assignees are
    /// silently dropped otherwise.
    pub assignees: Vec<User>,
    /// The desired state of the issue.
    pub state:     StateEdit,
    /// Number of the milestone to be associated with the
    /// issue. Set to `Option::None` to remove the milestone
    /// NOTE: Only users with push access can set labels for
    /// issues. Labels are silently dropped otherwise.
    pub milestone: Option<u64>,
    /// Labels to be associated to the issue. Set the Vec to
    /// empty to remove all labels. NOTE: Only users with
    /// push access can set labels for issues. Labels are
    /// silently dropped otherwise.
    pub labels:    Vec<String>
}

/// Sub-component for Issue
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Label {
    pub id:      u64,
    pub url:     String,
    pub name:    String,
    pub color:   String,
    pub default: bool
}

/// Sub-component for Issue
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Milestone {
    pub url:           String,
    pub html_url:      String,
    pub labels_url:    String,
    pub id:            u64,
    pub number:        String,
    pub state:         String,
    pub title:         String,
    pub description:   String,
    pub creator:       User,
    pub open_issues:   u64,
    pub closed_issues: u64,
    pub created_at:    String,
    pub updated_at:    String,
    pub closed_at:     String,
    pub due_on:        String
}

/// Sub-component for Issue
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PullRequest {
    pub url:       String,
    pub html_url:  String,
    pub diff_url:  String,
    pub patch_url: String
}

/// Sub-component of IssueEdit, it represents the desired
/// state of the issue.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum StateEdit {
    Open,
    Closed
}

/// Possible values for the filter parameter, it indicates
/// which kind of issues to return.
#[derive(Clone, Debug, PartialEq)]
pub enum Filter {
    /// Issues assigned to the authenticated user
    Assigned,
    /// Issues created by the authenticated user
    Created,
    /// Issues mentioning the authenticate user
    Mentioned,
    /// Issues that the authenticate user is subscribed to
    Subscribed,
    /// All issues the authenticate user can see.
    All
}

impl Filter {
    fn to_str(self) -> String {
        match self {
            Filter::All =>        "all".to_string(),
            Filter::Assigned =>   "assigned".to_string(),
            Filter::Created =>    "created".to_string(),
            Filter::Mentioned =>  "mentioned".to_string(),
            Filter::Subscribed => "subscribed".to_string()
        }
    }
}

/// Possible values for the state parameter, it indicates
/// the state of the issues to return.
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Open,
    Closed,
    All
}

impl State {
    fn to_str(self) -> String {
        match self {
            State::All    => "all".to_string(),
            State::Closed => "closed".to_string(),
            State::Open   => "open".to_string()
        }
    }
}

/// Possible values for the sorting of returned issues
#[derive(Clone, Debug, PartialEq)]
pub enum Sort {
    Created,
    Updated,
    Comments
}

impl Sort {
    fn to_str(self) -> String {
        match self {
            Sort::Comments => "comments".to_string(),
            Sort::Created  => "created".to_string(),
            Sort::Updated  => "updated".to_string()
        }
    }
}

/// Possible values for the sorting direction of returned
/// issues.
#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Ascending,
    Descending
}

impl Direction {
    fn to_str(self) -> String {
        match self {
            Direction::Ascending  => "asc".to_string(),
            Direction::Descending => "desc".to_string()
        }
    }
}

/// Possible values for the milestone parameter, it
/// indicates which issues to return by the milestone.
#[derive(Clone, Debug, PartialEq)]
pub enum MilestoneParam {
    /// Value of the number field in a milestone.
    Integer(u64),
    /// Name of the milestone, "*" can be used to get issues
    /// with any milestone associated to it.
    String(String),
    /// Issues without any milestone associated to it will
    /// be returned
    None

}

impl MilestoneParam {
    fn to_str(self) -> String {
        match self {
            MilestoneParam::Integer(param) => format!("{}", param),
            MilestoneParam::String(param)  => param,
            MilestoneParam::None           => "none".to_string()
        }
    }
}

/// Possible values for the assignee parameter, it
/// indicates which issues to return by the person assigned
/// to it.
#[derive(Clone, Debug, PartialEq)]
pub enum Assignee {
    /// Issues with the user assigned to it, "*" can be used
    /// to get issues assigned to any user.
    User(String),
    /// Issues without any assignee.
    None
}

impl Assignee {
    fn to_str(self) -> String {
        match self {
            Assignee::User(param) => param,
            Assignee::None        => "none".to_string()
        }
    }
}


////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait IssuesExt {

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#list-stargazers)\]
    /// Returns the list of issues.
    /// ## Endpoint:
    /// GET /issues
    /// ## Parameters
    /// * `filter`: Default: `Filter::Assigned`, indicates
    /// which kind of issues to return.
    /// * `state`: Default: `State::Open`, indicates
    /// the state of the issues to return.
    /// * `labels`: An Vec of labels.
    /// * `sort`: Default: `Sort::Created`, the requested
    /// sorting of returned issues.
    /// * `direction`: Default: `Direction::Descending`,
    /// the requested sorting direction of returned issues.
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_issues(&mut self,
                  filter: Option<Filter>,
                  state: Option<State>,
                  labels: Vec<String>,
                  sort: Option<Sort>,
                  direction: Option<Direction>,
                  since: String) -> Result<Vec<Issue>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#list-stargazers)\]
    /// Returns the list of issues assigned to an
    /// authenticated user.
    /// ## Endpoint:
    /// GET /user/issues
    /// ## Parameters
    /// * `filter`: Default: `Filter::Assigned`, indicates
    /// which kind of issues to return.
    /// * `state`: Default: `State::Open`, indicates
    /// the state of the issues to return.
    /// * `labels`: An Vec of labels.
    /// * `sort`: Default: `Sort::Created`, the requested
    /// sorting of returned issues.
    /// * `direction`: Default: `Direction::Descending`,
    /// the requested sorting direction of returned issues.
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_user_issues(&mut self,
                       filter: Option<Filter>,
                       state: Option<State>,
                       labels: Vec<String>,
                       sort: Option<Sort>,
                       direction: Option<Direction>,
                       since: String) -> Result<Vec<Issue>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#list-stargazers)\]
    /// Returns the list of issues assigned to an
    /// authenticated user.
    /// ## Endpoint:
    /// GET /orgs/:org/issues
    /// ## Parameters
    /// * `org`: Name of the organization.
    /// * `filter`: Default: `Filter::Assigned`, indicates
    /// which kind of issues to return.
    /// * `state`: Default: `State::Open`, indicates
    /// the state of the issues to return.
    /// * `labels`: An Vec of labels.
    /// * `sort`: Default: `Sort::Created`, the requested
    /// sorting of returned issues.
    /// * `direction`: Default: `Direction::Descending`,
    /// the requested sorting direction of returned issues.
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_orgs_org_issues(&mut self,
                           org: String,
                           filter: Option<Filter>,
                           state: Option<State>,
                           labels: Vec<String>,
                           sort: Option<Sort>,
                           direction: Option<Direction>,
                           since: String) -> Result<Vec<Issue>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/#list-issues-for-a-repository)\]
    /// Returns the list of issues for a repository.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/issues
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `milestone`: Milestone of the issues to be
    /// returned.
    /// * `state`: Default: `State::Open`, indicates
    /// the state of the issues to return.
    /// * `assignee`: Assignee of the issues to be returned.
    /// * `creator`: User that created the issue.
    /// * `labels`: An Vec of labels.
    /// * `sort`: Default: `Sort::Created`, the requested
    /// sorting of returned issues.
    /// * `direction`: Default: `Direction::Descending`,
    /// the requested sorting direction of returned issues.
    /// * `since`: Issues only updated at or after the time
    /// in ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn get_repos_owner_repo_issues(&mut self,
                                   owner: String,
                                   repo: String,
                                   milestone: MilestoneParam,
                                   state: Option<State>,
                                   assignee: Assignee,
                                   creator: String,
                                   mentioned: Option<String>,
                                   labels: Vec<String>,
                                   sort: Option<Sort>,
                                   direction: Option<Direction>,
                                   since: String) -> Result<Vec<Issue>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/#get-a-single-issue)\]
    /// Returns an issue for a repository.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/issues
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Issue number.
    fn get_repos_owner_repo_issues_number(&mut self, owner: String, repo: String, number: u64) -> Result<Issue, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/#create-an-issue)\]
    /// Creates an issue for a repository.
    /// Pull access to the repository is required.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/issues
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `issue`: Issue parameters.
    fn post_repos_owner_repo_issues(&mut self, owner: String, repo: String, issue: IssueCreate) -> Result<Issue, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/#edit-an-issue)\]
    /// Edits an issue for a repository.
    /// Pull access to the repository is required.
    /// ## Endpoint:
    /// PATCH /repos/:owner/:repo/issues/:number
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Issue number.
    /// * `issue`: Issue edit parameters.
    fn patch_repos_owner_repo_issues_number(&mut self, owner: String, repo: String, number: u64, issue: IssueEdit) -> Result<Issue, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/#lock-an-issue)\]
    /// Locks an issue.
    /// Pull access to the repository is required.
    /// ## Endpoint:
    /// PUT /repos/:owner/:repo/issues/:number/lock
    /// ## Parameters
    /// `number`: Issue number.
    fn put_repos_owner_repo_issues_number_lock(&mut self, owner: String, repo:String, number: u64) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/#lock-an-issue)\]
    /// Unlocks an issue.
    /// Pull access to the repository is required.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/issues/:number/lock
    /// ## Parameters
    /// `number`: Issue number.
    fn delete_repos_owner_repo_issues_number_lock(&mut self, owner: String, repo:String, number: u64) -> Result<(), error::Error>;

}

impl IssuesExt for Client {

    fn get_issues(&mut self,
                  filter: Option<Filter>,
                  state: Option<State>,
                  labels: Vec<String>,
                  sort: Option<Sort>,
                  direction: Option<Direction>,
                  since: String) -> Result<Vec<Issue>, error::Error>{

        let mut url = match Url::parse(&format!("{}/issues", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = filter {
                query_pairs.append_pair("filter", &param.to_str());
            }

            if let Some(param) = state {
                query_pairs.append_pair("state", &param.to_str());
            }

            if !labels.is_empty() {

                let mut param = String::new();
                for label in &labels {
                    param = format!("{},{}", param, label);
                }

                query_pairs.append_pair("labels", &param[..]);
            }

            if let Some(param) = sort {
                query_pairs.append_pair("sort", &param.to_str());
            }

            if let Some(param) = direction {
                query_pairs.append_pair("direction", &param.to_str());
            }

            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/issues?{}", url.as_str()))
    }

    fn get_user_issues(&mut self,
                       filter: Option<Filter>,
                       state: Option<State>,
                       labels: Vec<String>,
                       sort: Option<Sort>,
                       direction: Option<Direction>,
                       since: String) -> Result<Vec<Issue>, error::Error>{

        let mut url = match Url::parse(&format!("{}/user/issues", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = filter {
                query_pairs.append_pair("filter", &param.to_str());
            }

            if let Some(param) = state {
                query_pairs.append_pair("state", &param.to_str());
            }

            if !labels.is_empty() {

                let mut param = String::new();
                for label in &labels {
                    param = format!("{},{}", param, label);
                }

                query_pairs.append_pair("labels", &param[..]);
            }

            if let Some(param) = sort {
                query_pairs.append_pair("sort", &param.to_str());
            }

            if let Some(param) = direction {
                query_pairs.append_pair("direction", &param.to_str());
            }

            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/user/issues?{}", url.as_str()))
    }

    fn get_orgs_org_issues(&mut self,
                           org: String,
                           filter: Option<Filter>,
                           state: Option<State>,
                           labels: Vec<String>,
                           sort: Option<Sort>,
                           direction: Option<Direction>,
                           since: String) -> Result<Vec<Issue>, error::Error>{

        let mut url = match Url::parse(&format!("{}/orgs/{}/issues", self.api_url, org)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = filter {
                query_pairs.append_pair("filter", &param.to_str());
            }

            if let Some(param) = state {
                query_pairs.append_pair("state", &param.to_str());
            }

            if !labels.is_empty() {

                let mut param = String::new();
                for label in &labels {
                    param = format!("{},{}", param, label);
                }

                query_pairs.append_pair("labels", &param[..]);
            }

            if let Some(param) = sort {
                query_pairs.append_pair("sort", &param.to_str());
            }

            if let Some(param) = direction {
                query_pairs.append_pair("direction", &param.to_str());
            }

            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/orgs/{}/issues?{}", org, url.as_str()))
    }

    fn get_repos_owner_repo_issues(&mut self,
                                   owner: String,
                                   repo: String,
                                   milestone: MilestoneParam,
                                   state: Option<State>,
                                   assignee: Assignee,
                                   creator: String,
                                   mentioned: Option<String>,
                                   labels: Vec<String>,
                                   sort: Option<Sort>,
                                   direction: Option<Direction>,
                                   since: String) -> Result<Vec<Issue>, error::Error>{

        let mut url = match Url::parse(&format!("{}/repos/{}/{}/issues", self.api_url, owner, repo)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            query_pairs.append_pair("milestone", &milestone.to_str());

            if let Some(param) = state {
                query_pairs.append_pair("state", &param.to_str());
            }

            query_pairs.append_pair("assignee", &assignee.to_str());
            query_pairs.append_pair("creator", &creator[..]);

            if let Some(param) = mentioned {
                query_pairs.append_pair("mentioned", &param[..]);
            }

            if !labels.is_empty() {

                let mut param = String::new();
                for label in &labels {
                    param = format!("{},{}", param, label);
                }

                query_pairs.append_pair("labels", &param[..]);
            }

            if let Some(param) = sort {
                query_pairs.append_pair("sort", &param.to_str());
            }

            if let Some(param) = direction {
                query_pairs.append_pair("direction", &param.to_str());
            }

            query_pairs.append_pair("since", &since[..]);
        }

        utils::request_endpoint(self, format!("/repos/{}/{}/issues?{}", owner, repo, url.as_str()))
    }

    fn get_repos_owner_repo_issues_number(&mut self, owner: String, repo: String, number: u64) -> Result<Issue, error::Error> {
        utils::request_endpoint(self, format!("/repos/{}/{}/issues/{}", owner, repo, number))
    }

    fn post_repos_owner_repo_issues(&mut self, owner: String, repo: String, issue: IssueCreate) -> Result<Issue, error::Error> {

        //Create body
        let body_data = try!(serde_json::to_string(&issue).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/issues", owner, repo), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn patch_repos_owner_repo_issues_number(&mut self, owner: String, repo: String, number: u64, issue: IssueEdit) -> Result<Issue, error::Error> {

        //Create body
        let body_data = try!(serde_json::to_string(&issue).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/issues/{}", owner, repo, number), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn put_repos_owner_repo_issues_number_lock(&mut self, owner: String, repo: String, number: u64) -> Result<(), error::Error> {
        match self.put(format!("/repos/{}/{}/issues/{}/lock", owner, repo, number), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn delete_repos_owner_repo_issues_number_lock(&mut self, owner: String, repo: String, number: u64) -> Result<(), error::Error> {
        match self.delete(format!("/repos/{}/{}/issues/{}/lock", owner, repo, number), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }
}

//TODO: Custom media types support
//TODO: TESTS
