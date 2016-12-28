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

use hyper::{Error as HyperError, Url};
use serde_json;

use client::Client;
use issues::{Direction, Milestone, State};
use error;
use utils;

/// Possible values for the state parameter, it indicates
/// the state of the issues to return.
#[derive(Clone, Debug, PartialEq)]
pub enum Sort {
    Completeness,
    DueOn
}

impl Sort {
    fn to_str(self) -> String {
        match self {
            Sort::Completeness => "completeness".to_string(),
            Sort::DueOn        => "due_on".to_string(),
        }
    }
}

///Input parameter for creating a milestone
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MilestoneParam {
    /// Title of the milestone.
    title:       String,
    /// Default: `State::Open`, state of the milestone
    state:       Option<String>,
    /// Description of the milestone.
    description: String,
    /// Milestone's due date, a timestamp in ISO 8601
    /// (`YYYY-MM-DDTHH:MM:SSZ`) format.
    due_on:      String
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait MilestonesExt {

    /// \[[Reference](https://developer.github.com/v3/issues/milestones/#list-milestones-for-a-repository)\]
    /// Returns the list of milestones for a repository.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/milestones
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `state`: Default: `State::Open`, indicates
    /// the state of the milestones to return.
    /// * `sort`: Default: `Sort::DueOn`, the requested
    /// sorting of returned milestones.
    /// * `direction`: Default: `Direction::Ascending`,
    /// the requested sorting direction of returned
    /// milestones.
    fn get_repos_owner_repo_milestones(&mut self, owner: String, repo: String, state: Option<State>, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Milestone>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/milestones/#get-a-single-milestone)\]
    /// Returns the list of milestones for a repository.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/milestones/:number
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Number of the milestone.
    fn get_repos_owner_repo_milestones_number(&mut self, owner: String, repo: String, number: u64) -> Result<Milestone, error::Error>;

    /// \[[Reference](hhttps://developer.github.com/v3/issues/milestones/#create-a-milestone)\]
    /// Creates a milestone.
    /// ## Endpoint:
    /// POST /repos/:owner/:repo/milestones
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `milestone`: Input parameters, see
    /// `MilestoneParam`.
    fn post_repos_owner_repo_milestones(&mut self, owner: String, repo: String, milestone: MilestoneParam) -> Result<Milestone, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/milestones/#update-a-milestone)\]
    /// Updates a milestone.
    /// ## Endpoint:
    /// PATCH /repos/:owner/:repo/milestones/:number
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Number of the milestone to be updated.
    /// * `milestone`: Input parameters, see
    /// `MilestoneParam`.
    fn patch_repos_owner_repo_milestones_number(&mut self, owner: String, repo: String, number: u64, milestone: MilestoneParam) -> Result<Milestone, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/issues/milestones/#update-a-milestone)\]
    /// Deletes a milestone.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/milestones/:number
    /// ## Parameters
    /// * `owner`: Owner of the repo.
    /// * `repo`: Name of the repository.
    /// * `number`: Number of the milestone to be updated.
    fn delete_repos_owner_repo_milestones_number(&mut self, owner: String, repo: String, number: u64) -> Result<(), error::Error>;
}

impl MilestonesExt for Client {

    fn get_repos_owner_repo_milestones(&mut self, owner: String, repo: String, state: Option<State>, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Milestone>, error::Error>{

        let mut url = match Url::parse(&format!("{}/repos/{}/{}/milestones", self.api_url, owner, repo)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = state {
                query_pairs.append_pair("state", &param.to_str());
            }

            if let Some(param) = sort {
                query_pairs.append_pair("sort", &param.to_str());
            }

            if let Some(param) = direction {
                query_pairs.append_pair("direction", &param.to_str());
            }
        }

        utils::request_endpoint(self, format!("/repos/{}/{}/milestones?{}", owner, repo, url.as_str()))
    }

    fn get_repos_owner_repo_milestones_number(&mut self, owner: String, repo: String, number: u64) -> Result<Milestone, error::Error>{
        utils::request_endpoint(self, format!("/repos/{}/{}/milestones/{}", owner, repo, number))
    }

    fn post_repos_owner_repo_milestones(&mut self, owner: String, repo: String, milestone: MilestoneParam) -> Result<Milestone, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&milestone).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/repos/{}/{}/milestones", owner, repo), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn patch_repos_owner_repo_milestones_number(&mut self, owner: String, repo: String, number: u64, milestone: MilestoneParam) -> Result<Milestone, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&milestone).map_err(error::Error::Parsing));

        let mut response     = try!(self.patch_body(format!("/repos/{}/{}/milestones/{}", owner, repo, number), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn delete_repos_owner_repo_milestones_number(&mut self, owner: String, repo: String, number: u64) -> Result<(), error::Error>{
        match self.delete(format!("/repos/{}/{}/milestones/{}", owner, repo, number), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }
}

//TODO: TESTS
