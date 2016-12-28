// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Labels
//!
//! These are the responses and API call functions related
//! to the labels endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/issues/labels/

use serde_json;

use client::Client;
use error;
use utils;

/// Response element to the list labels endpoints.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Label {
    pub id:      u64,
    pub url:     String,
    pub name:    String,
    pub color:   String,
    pub default: bool
}

///Internal parameter
#[derive(Clone, Debug, PartialEq, Serialize)]
struct LabelParam {
    name:  String,
    color: String
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait LabelsExt {

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#list-all-labels-for-this-repository)\]
    /// Returns the list of labels for a repository.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/labels
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    fn get_repos_owner_repo_labels(&mut self, owner: String, repo: String) -> Result<Vec<Label>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#get-a-single-label)\]
    /// Returns a single label.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/labels/:name
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `name`: Name of the label.
    fn get_repos_owner_repo_labels_name(&mut self, owner: String, repo: String, name: String) -> Result<Label, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#create-a-label)\]
    /// Create sa label.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/labels
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `name`: Name of the label.
    /// * `color`: Color of the label in 6 character hex
    /// code, without the leading #, identifying the color.
    fn post_repos_owner_repo_labels(&mut self, owner: String, repo: String, name: String, color: String) -> Result<Label, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#update-a-label)\]
    /// Updates a label.
    /// ## Endpoint:
    /// PATCH /repos/:owner/:repo/labels
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `name`: Name of the label.
    /// * `new_name`: Name of the updated label.
    /// * `color`: Color of the label in 6 character hex
    /// code, without the leading #, identifying the color.
    fn patch_repos_owner_repo_labels_name(&mut self, owner: String, repo: String, name: String, new_name: String, color: String) -> Result<Label, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#delete-a-label)\]
    /// Deletes a label.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/labels/:name
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `name`: Name of the label.
    fn delete_repos_owner_repo_labels_name(&mut self, owner: String, repo: String, name: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#list-labels-on-an-issue)\]
    /// Returns a list of labels on an issue.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/issues/:number/labels
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Number of the issue.
    fn get_repos_owner_repo_issues_number_labels(&mut self, owner: String, repo: String, number: u64) -> Result<Label, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#add-labels-to-an-issue)\]
    /// Adds labels to an issue.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/issues/:number/labels
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Number of the issue.
    /// * `labels`: A vector of label names.
    fn post_repos_owner_repo_issues_number_labels(&mut self, owner: String, repo: String, number: u64, labels: Vec<String>) -> Result<Vec<Label>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#remove-a-label-from-an-issue)\]
    /// Removes a labels to an issue.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/issues/:number/labels/:name
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Number of the issue.
    /// * `name`: Name of the label.
    fn delete_repos_owner_repo_issues_number_labels_name(&mut self, owner: String, repo: String, number: u64, name: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#replace-all-labels-for-an-issue)\]
    /// Replaces all labels for an issue.
    /// ## Endpoint:
    /// PUT /repos/:owner/:repo/issues/:number/labels
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Number of the issue.
    /// * `labels`: A vector of label names.
    fn put_repos_owner_repo_issues_number_labels(&mut self, owner: String, repo: String, number: u64, labels: Vec<String>) -> Result<Vec<Label>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#remove-all-labels-from-an-issue)\]
    /// Removes all labels for an issue.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/issues/:number/labels
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Number of the issue.
    fn delete_repos_owner_repo_issues_number_labels(&mut self, owner: String, repo: String, number: u64) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/labels/#get-labels-for-every-issue-in-a-milestone)\]
    /// Removes all labels for an issue.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/issues/:number/labels
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Number of the issue.
    fn get_repos_owner_repo_milestones_number_labels(&mut self, owner: String, repo: String, number: u64) -> Result<Vec<Label>, error::Error>;
}

impl LabelsExt for Client {

    fn get_repos_owner_repo_labels(&mut self, owner: String, repo: String) -> Result<Vec<Label>, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/labels", owner, repo))
    }

    fn get_repos_owner_repo_labels_name(&mut self, owner: String, repo: String, name: String) -> Result<Label, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/labels/{}", owner, repo, name))
    }

    fn post_repos_owner_repo_labels(&mut self, owner: String, repo: String, name: String, color: String) -> Result<Label, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&LabelParam{
            name:  name,
            color: color
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/labels", owner, repo), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn patch_repos_owner_repo_labels_name(&mut self, owner: String, repo: String, name: String, new_name: String, color: String) -> Result<Label, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&LabelParam{
            name:  new_name,
            color: color
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.patch_body(format!("/repos/{}/{}/labels/{}", owner, repo, name), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn delete_repos_owner_repo_labels_name(&mut self, owner: String, repo: String, name: String) -> Result<(), error::Error>{
        match self.delete(format!("/repos/{}/{}/labels/{}", owner, repo, name), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn get_repos_owner_repo_issues_number_labels(&mut self, owner: String, repo: String, number: u64) -> Result<Label, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/issues/{}/labels", owner, repo, number))
    }

    fn post_repos_owner_repo_issues_number_labels(&mut self, owner: String, repo: String, number: u64, labels: Vec<String>) -> Result<Vec<Label>, error::Error>{

        let body_data = try!(serde_json::to_string(&labels).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/issues/{}/labels", owner, repo, number), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn delete_repos_owner_repo_issues_number_labels_name(&mut self, owner: String, repo: String, number: u64, name: String) -> Result<(), error::Error>{
        match self.delete(format!("/repos/{}/{}/issues/{}/labels/{}", owner, repo, number, name), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn put_repos_owner_repo_issues_number_labels(&mut self, owner: String, repo: String, number: u64, labels: Vec<String>) -> Result<Vec<Label>, error::Error>{

        let body_data = try!(serde_json::to_string(&labels).map_err(error::Error::Parsing));

        let mut response     = try!(self.put_body(format!("/repos/{}/{}/issues/{}/labels", owner, repo, number), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn delete_repos_owner_repo_issues_number_labels(&mut self, owner: String, repo: String, number: u64) -> Result<(), error::Error>{
        match self.delete(format!("/repos/{}/{}/issues/{}/labels", owner, repo, number), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn get_repos_owner_repo_milestones_number_labels(&mut self, owner: String, repo: String, number: u64) -> Result<Vec<Label>, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/milestones/{}/labels", owner, repo, number))
    }
}

//TODO: TEST
