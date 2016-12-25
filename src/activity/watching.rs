// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Watching
//!
//! These are the responses and API call functions related
//! to the watching endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/activity/watching/

use hyper::{Error as HyperError, Url};

use activity::common::Subscription;
use client::Client;
use common::{Repository, User};
use error;
use utils;


////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait WatchingExt {

    /// \[[Reference](https://developer.github.com/v3/activity/watching/#list-watchers)\]
    /// Returns the list of watchers.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/subscribers
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_repos_owner_repo_subscribers(&mut self, owner: String, repo: String) -> Result<User, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/watching/#list-repositories-being-watched)\]
    /// Returns the list of repositories being watched by a
    /// user.
    /// ## Endpoint:
    /// GET /users/:username/subscriptions
    /// ## Parameters:
    /// * `username`: Name of the user.
    fn get_users_username_subscription(&mut self, username: String) -> Result<Repository, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/watching/#list-repositories-being-watched)\]
    /// Returns the list of repositories being watched by an
    /// authenticated user.
    /// ## Endpoint:
    /// GET /user/subscriptions
    fn get_user_subscription(&mut self) -> Result<Repository, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/watching/#get-a-repository-subscription)\]
    /// Returns the repository subscription.
    /// ## Endpoint:
    /// GET /repos/:owner/:repo/subscription
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn get_repos_owner_repo_subscription(&mut self, owner: String, repo: String) -> Result<Subscription, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/watching/#get-a-repository-subscription)\]
    /// Sets a repository subscription.
    /// ## Endpoint:
    /// PUT /repos/:owner/:repo/subscription
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `subscribed`: Determines if notifications should be received from this repository.
    /// * `ignored`: Determines if all notifications should be blocked from this repository.
    fn put_repos_owner_repo_subscription(&mut self, owner: String, repo: String, subscribed: bool, ignored: bool) -> Result<Subscription, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/watching/#get-a-repository-subscription)\]
    /// Deletes a repository subscription/ unfollows
    /// a repository.
    /// ## Endpoint:
    /// DELETE /repos/:owner/:repo/subscription
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    fn delete_repos_owner_subscription(&mut self, owner: String, repo: String) -> Result<(), error::Error>;
}

impl WatchingExt for Client {

    fn get_repos_owner_repo_subscribers(&mut self, owner: String, repo: String) -> Result<User, error::Error> {
        utils::request_endpoint(self, format!("/repos/{}/{}/subscribers", owner, repo))
    }

    fn get_users_username_subscription(&mut self, username: String) -> Result<Repository, error::Error> {
        utils::request_endpoint(self, format!("/users/{}/subscriptions", username))
    }

    fn get_user_subscription(&mut self) -> Result<Repository, error::Error> {
        utils::request_endpoint(self, "/user/subscriptions".into())
    }

    fn get_repos_owner_repo_subscription(&mut self, owner: String, repo: String) -> Result<Subscription, error::Error> {
        utils::request_endpoint(self, format!("/repos/{}/{}/subscription", owner, repo))
    }

    fn put_repos_owner_repo_subscription(&mut self, owner: String, repo: String, subscribed: bool, ignored: bool) -> Result<Subscription, error::Error> {

        let mut url = match Url::parse(&format!("{}/repos/{}/{}/subscription", self.api_url, owner, repo)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            query_pairs.append_pair("subscribed", &format!("{}", subscribed)[..]);
            query_pairs.append_pair("ignored",    &format!("{}", ignored)[..]);
        }

        utils::request_endpoint(self, format!("/repos/{}/{}/subscription?{}", owner, repo, url.query().unwrap()))
    }

    fn delete_repos_owner_subscription(&mut self, owner: String, repo: String) -> Result<(), error::Error> {
        match self.delete(format!("/repos/{}/{}/subscription", owner, repo), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }
}

//TODO?: Add legacy endpoints
//TODO:  TESTS
