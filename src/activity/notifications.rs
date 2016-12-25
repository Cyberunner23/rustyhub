// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Notifications
//!
//! These are the responses and API call functions related
//! to the notifications endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/activity/notifications/

use hyper::{Error as HyperError, Url};
use serde_json;

use activity::common::Subscription;
use common::Repository;
use client::Client;
use error;
use utils;

///Response to most notifications requests
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Notification {
    pub id:           String,
    pub repository:   Repository,
    pub subject:      Subject,
    pub reason:       Option<String>,
    pub unread:       bool,
    pub updated_at:   String,
    pub last_read_at: Option<String>,
    pub url:          String
}

/// Sub-component of the Notification response
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Subject {
    pub title: String,
    pub url:   String,
    pub latest_comment_url: String,
    #[serde(rename = "type")]
    pub subject_type: String
}


////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait NotificationsExt {

    /// \[[Reference](https://developer.github.com/v3/activity/notifications/#list-your-notifications)\]
    /// Returns the list of notifications.
    /// ## Endpoint:
    /// GET /notifications
    /// ## Parameters:
    /// * `all`: Default: false, Show notification marked
    /// as read.
    /// * `participating`: Default: false, Only show
    /// notifications where the user is directly
    /// participating or mentioned.
    /// * `since`: Name of the repository.
    /// * `before`: Name of the repository.
    fn get_notifications(&mut self,
                         all: Option<bool>,
                         participating: Option<bool>,
                         since: Option<String>,
                         before: Option<String>) -> Result<Vec<Notification>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/notifications/#list-your-notifications-in-a-repository)\]
    /// Returns the list of notifications in a repository.
    /// ## Endpoint:
    /// GET /notifications
    /// ## Parameters:
    /// * `all`: Owner of the repository.
    /// * `participating`: Name of the repository.
    /// * `since`: Name of the repository.
    /// * `before`: Name of the repository.
    fn get_repos_owner_repo_notifications(&mut self,
                                          owner: String,
                                          repo: String,
                                          all: Option<bool>,
                                          participating: Option<bool>,
                                          since: Option<String>,
                                          before: Option<String>) -> Result<Vec<Notification>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/notifications/#mark-as-read)\]
    /// Marks notifications as read.
    /// ## Endpoint:
    /// PUT /notifications
    /// ## Parameters:
    /// * `last_read_at`: Default: "Time.now", The last
    /// point at which notifications were checked in the
    /// ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn put_notifications(&mut self, last_read_at: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/notifications/#mark-notifications-as-read-in-a-repository)\]
    /// Marks notifications in a repository as read.
    /// ## Endpoint:
    /// PUT /repos/:owner/:repo/notifications
    /// ## Parameters:
    /// * `owner`: Owner of the repository.
    /// * `repo`: Name of the repository.
    /// * `last_read_at`: Default: "Time.now", The last
    /// point at which notifications were checked in the
    /// ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) format.
    fn put_repos_owner_repo_notifications(&mut self, owner: String, repo: String, last_read_at: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/notifications/#view-a-single-thread)\]
    /// Returns a single thread.
    /// ## Endpoint:
    /// GET /notifications/threads/:id
    /// ## Parameters:
    /// * `id`: Thread id.
    fn get_notifications_threads_id(&mut self, id: String) -> Result<Notification, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/notifications/#mark-a-thread-as-read)\]
    /// Marks a single as thread.
    /// ## Endpoint:
    /// PATCH /notifications/threads/:id
    /// ## Parameters:
    /// * `id`: Thread id.
    fn patch_botifications_threads_id(&mut self, id: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/notifications/#get-a-thread-subscription)\]
    /// Returns a single as thread subscription.
    /// ## Endpoint:
    /// GET /notifications/threads/:id/subscription
    /// ## Parameters:
    /// * `id`: Thread id.
    fn get_notifications_threads_id_subscription(&mut self, id: String) -> Result<Subscription, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/notifications/#set-a-thread-subscription)\]
    /// Sets a thread subscription.
    /// ## Endpoint:
    /// PUT /notifications/threads/:id/subscription
    /// ## Parameters:
    /// * `id`: Thread id.
    /// * `subscribed`: Determines if notifications should be received from this thread.
    /// * `ignored`: Determines if all notifications should be blocked from this thread.
    fn put_notifications_threads_id_subscription(&mut self, id: String, subscribed: bool, ignored: bool) -> Result<Subscription, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/activity/notifications/#delete-a-thread-subscription)\]
    /// Deletes a thread subscription.
    /// ## Endpoint:
    /// DELETE /notifications/threads/:id/subscription
    /// ## Parameters:
    /// * `id`: Thread id.
    ///Reference:
    fn delete_notifications_threads_id_subscription(&mut self, id: String) -> Result<(), error::Error>;

}

impl NotificationsExt for Client {

    fn get_notifications(&mut self, all: Option<bool>, participating: Option<bool>, since: Option<String>, before: Option<String>) -> Result<Vec<Notification>, error::Error> {

        let mut url = match Url::parse(&format!("{}/notifications", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = all {
                query_pairs.append_pair("all", &format!("{}", param)[..]);
            }

            if let Some(param) = participating {
                query_pairs.append_pair("participating", &format!("{}", param)[..]);
            }

            if let Some(param) = since {
                query_pairs.append_pair("since", &param[..]);
            }

            if let Some(param) = before {
                query_pairs.append_pair("before", &param[..]);
            }
        }
        utils::request_endpoint(self, format!("/notifications?{}", url.query().unwrap()))
    }

    fn get_repos_owner_repo_notifications(&mut self, owner: String, repo: String, all: Option<bool>, participating: Option<bool>, since: Option<String>, before: Option<String>) -> Result<Vec<Notification>, error::Error> {

        let mut url = match Url::parse(&format!("{}/repos/{}/{}/notifications", self.api_url, owner, repo)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = all {
                query_pairs.append_pair("all", &format!("{}", param)[..]);
            }

            if let Some(param) = participating {
                query_pairs.append_pair("participating", &format!("{}", param)[..]);
            }

            if let Some(param) = since {
                query_pairs.append_pair("since", &param[..]);
            }

            if let Some(param) = before {
                query_pairs.append_pair("before", &param[..]);
            }
        }

        utils::request_endpoint(self, format!("/repos/{}/{}/notifications?{}", owner, repo, url.query().unwrap()))
    }

    fn put_notifications(&mut self, last_read_at: String) -> Result<(), error::Error> {

        let mut url = match Url::parse(&format!("{}/notifications", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            query_pairs.append_pair("last_read_at", &last_read_at[..]);
        }

        match self.put(format!("/notifications?{}", url.query().unwrap()), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn put_repos_owner_repo_notifications(&mut self, owner: String, repo: String, last_read_at: String) -> Result<(), error::Error> {

        let mut url = match Url::parse(&format!("{}/repos/{}/{}notifications", self.api_url, owner, repo)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            query_pairs.append_pair("last_read_at", &last_read_at[..]);
        }

        match self.put(format!("/repos/{}/{}notifications?{}", owner, repo, url.query().unwrap()), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn get_notifications_threads_id(&mut self, id: String) -> Result<Notification, error::Error> {
        utils::request_endpoint(self, format!("/notifications/threads/{}", id))
    }

    fn patch_botifications_threads_id(&mut self, id: String) -> Result<(), error::Error> {
        match self.patch(format!("/notifications/threads/{}", id), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn get_notifications_threads_id_subscription(&mut self, id: String) -> Result<Subscription, error::Error> {
        utils::request_endpoint(self, format!("/notifications/threads/{}/subscription", id))
    }

    fn put_notifications_threads_id_subscription(&mut self, id: String, subscribed: bool, ignored: bool) -> Result<Subscription, error::Error> {

        let mut url = match Url::parse(&format!("{}/notifications/threads/{}/subscription", self.api_url, id)[..]) {
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

        let mut response     = try!(self.put(format!("/notifications/threads/{}/subscription?{}", id, url.query().unwrap()), None));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)

    }

    fn delete_notifications_threads_id_subscription(&mut self, id: String) -> Result<(), error::Error> {
        match self.delete(format!("/notifications/threads/{}/subscription", id), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }
}

//TODO:TESTS
