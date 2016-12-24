// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use hyper::{Error as HyperError, Url};
use serde_json;

use activity::common::Subscription;
use common::Repository;
use client::Client;
use error;

///Reference: https://developer.github.com/v3/activity/notifications/

///Response to /notifications
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Subject {
    pub title: String,
    pub url:   String,
    pub latest_comment_url: String,
    #[serde(rename = "type")]
    pub subject_type: String
}


////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

///Reference: https://developer.github.com/v3/activity/notifications/#list-your-notifications
pub fn get_notifications(client: &mut Client,
                         all: Option<bool>,
                         participating: Option<bool>,
                         since: Option<String>,
                         before: Option<String>) -> Result<Vec<Notification>, error::Error> {

    let mut url = match Url::parse(&format!("{}/notifications", client.api_url)[..]) {
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

    let mut response     = try!(client.get(format!("/notifications?{}", url.query().unwrap()), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/notifications/#list-your-notifications-in-a-repository
pub fn get_repos_owner_repo_notifications(client: &mut Client,
                         owner: String,
                         repo: String,
                         all: Option<bool>,
                         participating: Option<bool>,
                         since: Option<String>,
                         before: Option<String>) -> Result<Vec<Notification>, error::Error> {

    let mut url = match Url::parse(&format!("{}/repos/{}/{}/notifications", client.api_url, owner, repo)[..]) {
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

    let mut response     = try!(client.get(format!("/repos/{}/{}/notifications?{}", owner, repo, url.query().unwrap()), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/notifications/#mark-as-read
pub fn put_notifications(client: &mut Client, last_read_at: String) -> Result<(), error::Error> {

    let mut url = match Url::parse(&format!("{}/notifications", client.api_url)[..]) {
        Ok(url)  => url,
        Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
    };

    //Limits the scope of the mutable borrow
    {
        let mut query_pairs = url.query_pairs_mut();
        query_pairs.clear();
        query_pairs.append_pair("last_read_at", &last_read_at[..]);
    }

    match client.put(format!("/notifications?{}", url.query().unwrap()), None) {
        Ok(_)    => Ok(()),
        Err(err) => Err(err)
    }
}

///Reference: https://developer.github.com/v3/activity/notifications/#mark-notifications-as-read-in-a-repository
pub fn put_repos_owner_repo_notifications(client: &mut Client, owner: String, repo: String, last_read_at: String) -> Result<(), error::Error> {

    let mut url = match Url::parse(&format!("{}/repos/{}/{}notifications", client.api_url, owner, repo)[..]) {
        Ok(url)  => url,
        Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
    };

    //Limits the scope of the mutable borrow
    {
        let mut query_pairs = url.query_pairs_mut();
        query_pairs.clear();
        query_pairs.append_pair("last_read_at", &last_read_at[..]);
    }

    match client.put(format!("/repos/{}/{}notifications?{}", owner, repo, url.query().unwrap()), None) {
        Ok(_)    => Ok(()),
        Err(err) => Err(err)
    }
}

///Reference: https://developer.github.com/v3/activity/notifications/#view-a-single-thread
pub fn get_notifications_threads_id(client: &mut Client, id: String) -> Result<Notification, error::Error> {
    let mut response     = try!(client.get(format!("/notifications/threads/{}", id), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/notifications/#mark-a-thread-as-read
pub fn patch_botifications_threads_id(client: &mut Client, id: String) -> Result<(), error::Error> {
    match client.patch(format!("/notifications/threads/{}", id), None) {
        Ok(_)    => Ok(()),
        Err(err) => Err(err)
    }
}

///Reference: https://developer.github.com/v3/activity/notifications/#get-a-thread-subscription
pub fn get_notifications_threads_id_subscription(client: &mut Client, id: String) -> Result<Subscription, error::Error> {
    let mut response     = try!(client.get(format!("/notifications/threads/{}/subscription", id), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/notifications/#set-a-thread-subscription
pub fn put_notifications_threads_id_subscription(client: &mut Client, id: String, subscribed: bool, ignored: bool) -> Result<Subscription, error::Error> {

    let mut url = match Url::parse(&format!("{}/notifications/threads/{}/subscription", client.api_url, id)[..]) {
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

    let mut response     = try!(client.put(format!("/notifications/threads/{}/subscription?{}", id, url.query().unwrap()), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/notifications/#delete-a-thread-subscription
pub fn delete_notifications_threads_id_subscription(client: &mut Client, id: String) -> Result<(), error::Error> {
    match client.delete(format!("/notifications/threads/{}/subscription", id), None) {
        Ok(_)    => Ok(()),
        Err(err) => Err(err)
    }
}

//TODO:TESTS
