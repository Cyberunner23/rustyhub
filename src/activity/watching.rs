// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use hyper::{Error as HyperError, Url};
use serde_json;

use activity::common::Subscription;
use client::Client;
use common::{Repository, User};
use error;

///Reference: https://developer.github.com/v3/activity/watching/


///Reference: https://developer.github.com/v3/activity/watching/#list-watchers
pub fn get_repos_owner_repo_subscribers(client: &mut Client, owner: String, repo: String) -> Result<User, error::Error> {
    let mut response     = try!(client.get(format!("/repos/{}/{}/subscribers", owner, repo), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/watching/#list-repositories-being-watched
pub fn get_users_username_subscription(client: &mut Client, username: String) -> Result<Repository, error::Error> {
    let mut response     = try!(client.get(format!("/users/{}/subscriptions", username), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

pub fn get_user_subscription(client: &mut Client) -> Result<Repository, error::Error> {
    let mut response     = try!(client.get("/user/subscriptions".to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/watching/#get-a-repository-subscription
pub fn get_repos_owner_repo_subscription(client: &mut Client, owner: String, repo: String) -> Result<Subscription, error::Error> {
    let mut response     = try!(client.get(format!("/repos/{}/{}/subscription", owner, repo), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/watching/#set-a-repository-subscription
pub fn put_repos_owner_repo_subscription(client: &mut Client, owner: String, repo: String, subscribed: bool, ignored: bool) -> Result<Subscription, error::Error> {

    let mut url = match Url::parse(&format!("{}/repos/{}/{}/subscription", client.api_url, owner, repo)[..]) {
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

    let mut response     = try!(client.put(format!("/repos/{}/{}/subscription?{}", owner, repo, url.query().unwrap()), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/watching/#delete-a-repository-subscription
pub fn delete_repos_owner_subscription(client: &mut Client, owner: String, repo: String) -> Result<(), error::Error> {
    match client.delete(format!("/repos/{}/{}/subscription", owner, repo), None) {
        Ok(_)    => Ok(()),
        Err(err) => Err(err)
    }
}

//TODO?: Add legacy endpoints
//TODO:  TESTS