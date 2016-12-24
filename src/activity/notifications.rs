// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use hyper::{Error as HyperError, Url};
use serde_json;

use client::Client;
use error;

///Reference: https://developer.github.com/v3/activity/notifications/

///Response to /notifications
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Notification {
    id:           String,
    repository:   Repository,
    subject:      Subject,
    reason:       String,
    unread:       bool,
    updated_at:   String,
    last_read_at: Option<String>,
    url:          String
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Subject {
    title: String,
    url:   String,
    latest_comment_url: String,
    #[serde(rename = "type")]
    subject_type: String
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Repository {
    id:                u64,
    name:              String,
    full_name:         String,
    owner:             Owner,
    description:       String,
    private:           bool,
    fork:              bool,
    url:               String,
    html_url:          String,
    forks_url:         Option<String>,
    keys_url:          Option<String>,
    collaborators_url: Option<String>,
    teams_url:         Option<String>,
    hooks_url:         Option<String>,
    issue_events_url:  Option<String>,
    events_url:        Option<String>,
    assignees_url:     Option<String>,
    branches_url:      Option<String>,
    tags_url:          Option<String>,
    blobs_url:         Option<String>,
    git_tags_url:      Option<String>,
    git_refs_url:      Option<String>,
    trees_url:         Option<String>,
    statuses_url:      Option<String>,
    languages_url:     Option<String>,
    stargazers_url:    Option<String>,
    contributors_url:  Option<String>,
    subscribers_url:   Option<String>,
    subscription_url:  Option<String>,
    commits_url:       Option<String>,
    git_commits_url:   Option<String>,
    comments_url:      Option<String>,
    issue_comment_url: Option<String>,
    contents_url:      Option<String>,
    compare_url:       Option<String>,
    merges_url:        Option<String>,
    archive_url:       Option<String>,
    downloads_url:     Option<String>,
    issues_url:        Option<String>,
    pulls_url:         Option<String>,
    milestones_url:    Option<String>,
    notifications_url: Option<String>,
    labels_url:        Option<String>,
    releases_url:      Option<String>,
    deployments_url:   Option<String>
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Owner {
    login:               String,
    id:                  u64,
    avatar_url:          String,
    gravatar_id:         String,
    url:                 String,
    html_url:            String,
    followers_url:       String,
    following_url:       String,
    gists_url:           String,
    starred_url:         String,
    subscriptions_url:   String,
    organizations_url:   String,
    repos_url:           String,
    events_url:          String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type:           String,
    site_admin:          bool
}

///Response to subscription endpoints
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Subscription {
    subscribed: bool,
    ignored:    bool,
    //reason:   null (seems to always be null)
    created_at: String,
    url:        String,
    thread_url: String
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

    let mut response     = try!(client.get(url.as_str().to_string(), None));
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

    let mut response     = try!(client.get(url.as_str().to_string(), None));
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

    match client.put(url.as_str().to_string(), None) {
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

    match client.put(url.as_str().to_string(), None) {
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

    let mut response     = try!(client.put(url.as_str().to_string(), None));
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
