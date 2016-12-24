// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use hyper::{Error as HyperError, Url};
use hyper::header::{Accept, ContentLength, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde_json;

use common::{Repository, User};
use client::Client;
use error;

///Reference: https://developer.github.com/v3/activity/starring/

///Response returned by the timestamp variant of list stargazers
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ListStarTimeStamp {
    pub starred_at: String,
    pub user:       User
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ListRepoStarTimeStamp {
    pub starred_at: String,
    pub repo:       Repository
}

pub enum Sort {
    Created,
    Updated
}

pub enum Direction {
    Ascending,
    Descending
}


////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

///Reference: https://developer.github.com/v3/activity/starring/#list-stargazers
pub fn get_repos_owner_repo_stargazers(client: &mut Client, owner: String, repo: String) -> Result<Vec<User>, error::Error> {
    let mut response     = try!(client.get(format!("/repos/{}/{}/stargazers", owner, repo).to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Timestamp variant
pub fn get_repos_owner_repo_stargazers_timestamp(client: &mut Client, owner: String, repo: String) -> Result<Vec<ListStarTimeStamp>, error::Error> {

    let mut header = client.get_default_header();
    header.remove::<Accept>();
    header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3.star+json".to_string()), vec![]))]));

    let mut response     = try!(client.get(format!("/repos/{}/{}/stargazers", owner, repo).to_string(), Some(header)));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/starring/#list-repositories-being-starred
pub fn get_users_username_starred(client: &mut Client, username: String, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Repository>, error::Error> {

    let mut url = match Url::parse(&format!("{}/users/{}/starred", client.api_url, username)[..]) {
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

    let mut response     = try!(client.get(format!("/users/{}/starred?{}", username, url.query().unwrap()), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)

}

pub fn get_users_starred(client: &mut Client, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Repository>, error::Error> {

    let mut url = match Url::parse(&format!("{}/user/starred", client.api_url)[..]) {
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

    let mut response     = try!(client.get(format!("/users/starred?{}", url.query().unwrap()), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)

}

///Timestamp variants
pub fn get_users_username_starred_timestamp(client: &mut Client, username: String, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<ListRepoStarTimeStamp>, error::Error> {

    let mut url = match Url::parse(&format!("{}/users/{}/starred", client.api_url, username)[..]) {
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

    let mut header = client.get_default_header();
    header.remove::<Accept>();
    header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3.star+json".to_string()), vec![]))]));

    let mut response     = try!(client.get(format!("/users/{}/starred?{}", username, url.query().unwrap()), Some(header)));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)

}

pub fn get_users_starred_timestamp(client: &mut Client, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<ListRepoStarTimeStamp>, error::Error> {

    let mut url = match Url::parse(&format!("{}/user/starred", client.api_url)[..]) {
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

    let mut header = client.get_default_header();
    header.remove::<Accept>();
    header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3.star+json".to_string()), vec![]))]));

    let mut response     = try!(client.get(format!("/user/starred?{}", url.query().unwrap()), Some(header)));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)

}

///Reference: https://developer.github.com/v3/activity/starring/#check-if-you-are-starring-a-repository
pub fn get_user_starred_owner_repo(client: &mut Client, owner: String, repo: String) -> Result<(), error::Error> {
    let mut response     = try!(client.get(format!("/user/starred/{}/{}", owner, repo), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/activity/starring/#star-a-repository
pub fn put_user_starred_owner_repo(client: &mut Client, owner: String, repo: String) -> Result<(), error::Error> {

    let mut header = client.get_default_header();
    header.set(ContentLength(0u64));

    match client.get(format!("/user/starred/{}/{}", owner, repo), Some(header)) {
        Ok(_)    => Ok(()),
        Err(err) => Err(err)
    }
}

///Reference: https://developer.github.com/v3/activity/starring/#unstar-a-repository
pub fn delete_user_starred_owner_repo(client: &mut Client, owner: String, repo: String) -> Result<(), error::Error> {
    match client.delete(format!("/user/starred/{}/{}", owner, repo), None) {
        Ok(_)    => Ok(()),
        Err(err) => Err(err)
    }
}

//TODO: TESTS