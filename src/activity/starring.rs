// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Starred
//!
//! These are the responses and API call functions related
//! to the starred endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/activity/starring/

use hyper::{Error as HyperError, Uri};
use hyper::header::{Accept, ContentLength, qitem};
use hyper::mime::{Mime};

use common::{Repository, User};
use client::Client;
use error;
use utils;

///Response returned by the timestamp variant of list stargazers
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ListStarTimeStamp {
    pub starred_at: String,
    pub user:       User
}

///Response returned by the timestamp variant of list
/// stargazers
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ListRepoStarTimeStamp {
    pub starred_at: String,
    pub repo:       Repository
}

///Enum for input of sorting in get user starred endpoints
pub enum Sort {
    Created,
    Updated
}

///Enum for input of dorting direction in get user starred
/// endpoints
pub enum Direction {
    Ascending,
    Descending
}


////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait StarringExt {

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#list-stargazers)\]
    /// Returns the list of stargazers.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/stargazers
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_repos_owner_repo_stargazers(&mut self, owner: String, repo: String) -> Result<Vec<User>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#list-stargazers)\]
    /// Returns the list of stargazers with timestamp.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/stargazers
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_repos_owner_repo_stargazers_timestamp(&mut self, owner: String, repo: String) -> Result<Vec<ListStarTimeStamp>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#list-repositories-being-starred)\]
    /// Returns the list of repositories being starred by
    /// a user.
    /// ## Endpoint:
    /// GET /users/:username/starred
    /// ## Parameters:
    /// * `username`: Name of the user.
    /// * `sort`: Default: `Sort::Created`, sort by when the
    /// repository was starred (`Sort::Created`) or by when
    /// the repository was last pushed to (`Sort::Updated`).
    /// * `direction`: Default: `Direction::Ascending`, sort
    /// in ascending (`Direction::Ascending`) or in
    /// descending (`Direction::Descending`) order.
    fn get_users_username_starred(&mut self, username: String, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Repository>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#list-repositories-being-starred)\]
    /// Returns the list of repositories with timestamp
    /// being starred by a user.
    /// ## Endpoint:
    /// GET /users/:username/starred
    /// ## Parameters:
    /// * `username`: Name of the user.
    /// * `sort`: Default: `Sort::Created`, sort by when the
    /// repository was starred (`Sort::Created`) or by when
    /// the repository was last pushed to (`Sort::Updated`).
    /// * `direction`: Default: `Direction::Ascending`, sort
    /// in ascending (`Direction::Ascending`) or in
    /// descending (`Direction::Descending`) order.
    fn get_users_username_starred_timestamp(&mut self, username: String, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<ListRepoStarTimeStamp>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#list-repositories-being-starred)\]
    /// Returns the list of repositories being starred by
    /// an authenticated user.
    /// ## Endpoint:
    /// GET /user/starred
    /// ## Parameters:
    /// * `sort`: Default: `Sort::Created`, sort by when the
    /// repository was starred (`Sort::Created`) or by when
    /// the repository was last pushed to (`Sort::Updated`).
    /// * `direction`: Default: `Direction::Ascending`, sort
    /// in ascending (`Direction::Ascending`) or in
    /// descending (`Direction::Descending`) order.
    fn get_user_starred(&mut self, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Repository>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#list-repositories-being-starred)\]
    /// Returns the list of repositories with timestamp
    /// being starred by an authenticated user.
    /// ## Endpoint:
    /// GET /user/starred
    /// ## Parameters:
    /// * `sort`: Default: `Sort::Created`, sort by when the
    /// repository was starred (`Sort::Created`) or by when
    /// the repository was last pushed to (`Sort::Updated`).
    /// * `direction`: Default: `Direction::Ascending`, sort
    /// in ascending (`Direction::Ascending`) or in
    /// descending (`Direction::Descending`) order.
    fn get_user_starred_timestamp(&mut self, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<ListRepoStarTimeStamp>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#check-if-you-are-starring-a-repository)\]
    /// Returns whether an authenticated user is starring a
    /// repository.
    /// ## Endpoint:
    /// GET /user/starred/:owner/:repo
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// ## Return Values:
    /// * If repository is starred: returns Ok(())
    /// * If repository is not starred: returns Error::Github
    fn get_user_starred_owner_repo(&mut self, owner: String, repo: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#star-a-repository)\]
    /// Stars a repository.
    /// ## Endpoint:
    /// PUT /user/starred/:owner/:repo
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn put_user_starred_owner_repo(&mut self, owner: String, repo: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/starring/#unstar-a-repository)\]
    /// Unstars a repository.
    /// ## Endpoint:
    /// DELETE /user/starred/:owner/:repo
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn delete_user_starred_owner_repo(&mut self, owner: String, repo: String) -> Result<(), error::Error>;
}

impl StarringExt for Client {

    fn get_repos_owner_repo_stargazers(&mut self, owner: String, repo: String) -> Result<Vec<User>, error::Error> {
        utils::request_endpoint(self, format!("/repos/{}/{}/stargazers", owner, repo))
    }

    fn get_repos_owner_repo_stargazers_timestamp(&mut self, owner: String, repo: String) -> Result<Vec<ListStarTimeStamp>, error::Error> {

        let mut header = self.get_default_headers();
        header.remove::<Accept>();
        header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3.star+json".to_string()), vec![]))]));

        utils::request_endpoint_with_headers(self, format!("/repos/{}/{}/stargazers", owner, repo), Some(header))
    }

    fn get_users_username_starred(&mut self, username: String, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Repository>, error::Error> {

        let mut url = match Uri::to_str(&format!("{}/users/{}/starred", self.api_url, username)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = sort {
                let sorting = match param {
                    Sort::Created => "created",
                    Sort::Updated => "updated",
                };
                query_pairs.append_pair("sort", &sorting[..]);
            }

            if let Some(param) = direction {
                let dir = match param {
                    Direction::Ascending  => "asc",
                    Direction::Descending => "desc",
                };
                query_pairs.append_pair("direction", &dir[..]);
            }

        }

        utils::request_endpoint(self, format!("/users/{}/starred?{}", username, url.query().unwrap()))
    }

    fn get_user_starred(&mut self, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Repository>, error::Error> {

        let mut url = match Uri::to_str(&format!("{}/user/starred", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = sort {
                let sorting = match param {
                    Sort::Created => "created",
                    Sort::Updated => "updated",
                };
                query_pairs.append_pair("sort", &sorting[..]);
            }

            if let Some(param) = direction {
                let dir = match param {
                    Direction::Ascending  => "asc",
                    Direction::Descending => "desc",
                };
                query_pairs.append_pair("direction", &dir[..]);
            }

        }

        utils::request_endpoint(self, format!("/users/starred?{}", url.query().unwrap()))

    }

    fn get_users_username_starred_timestamp(&mut self, username: String, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<ListRepoStarTimeStamp>, error::Error> {

        let mut url = match Uri::to_str(&format!("{}/users/{}/starred", self.api_url, username)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = sort {
                let sorting = match param {
                    Sort::Created => "created",
                    Sort::Updated => "updated",
                };
                query_pairs.append_pair("sort", &sorting[..]);
            }

            if let Some(param) = direction {
                let dir = match param {
                    Direction::Ascending  => "asc",
                    Direction::Descending => "desc",
                };
                query_pairs.append_pair("direction", &dir[..]);
            }

        }

        let mut header = self.get_default_headers();
        header.remove::<Accept>();
        header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3.star+json".to_string()), vec![]))]));

        utils::request_endpoint_with_headers(self, format!("/users/{}/starred?{}", username, url.query().unwrap()), Some(header))

    }

    fn get_user_starred_timestamp(&mut self, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<ListRepoStarTimeStamp>, error::Error> {

        let mut url = match Uri::to_str(&format!("{}/user/starred", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = sort {
                let sorting = match param {
                    Sort::Created => "created",
                    Sort::Updated => "updated",
                };
                query_pairs.append_pair("sort", &sorting[..]);
            }

            if let Some(param) = direction {
                let dir = match param {
                    Direction::Ascending  => "asc",
                    Direction::Descending => "desc",
                };
                query_pairs.append_pair("direction", &dir[..]);
            }

        }

        let mut header = self.get_default_headers();
        header.remove::<Accept>();
        header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3.star+json".to_string()), vec![]))]));

        utils::request_endpoint_with_headers(self, format!("/user/starred?{}", url.query().unwrap()), Some(header))

    }

    fn get_user_starred_owner_repo(&mut self, owner: String, repo: String) -> Result<(), error::Error> {
        utils::request_endpoint(self, format!("/user/starred/{}/{}", owner, repo))
    }

    fn put_user_starred_owner_repo(&mut self, owner: String, repo: String) -> Result<(), error::Error> {

        let mut header = self.get_default_headers();
        header.set(ContentLength(0u64));

        match self.get(format!("/user/starred/{}/{}", owner, repo), Some(header)) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn delete_user_starred_owner_repo(&mut self, owner: String, repo: String) -> Result<(), error::Error> {
        match self.delete(format!("/user/starred/{}/{}", owner, repo), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }
}

//TODO: TESTS
