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

use client::Client;
use error;

///Reference: https://developer.github.com/v3/activity/starring/

///Response returned by the timestamp variant of list stargazers
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ListStarTimeStamp {
    starred_at: String,
    user:       User
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ListRepoStarTimeStamp {
    starred_at: String,
    repo:       Repo
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Repo {
    id:                u64,
    owner:             User,
    name:              String,
    full_name:         String,
    description:       Option<String>,
    private:           bool,
    fork:              bool,
    url:               String,
    html_url:          String,
    archive_url:       String,
    assignees_url:     String,
    blobs_url:         String,
    branches_url:      String,
    clone_url:         String,
    collaborators_url: String,
    comments_url:      String,
    commits_url:       String,
    compare_url:       String,
    contents_url:      String,
    contributors_url:  String,
    deployments_url:   String,
    downloads_url:     String,
    events_url:        String,
    forks_url:         String,
    git_commits_url:   String,
    git_refs_url:      String,
    git_tags_url:      String,
    git_url:           String,
    hooks_url:         String,
    issue_comment_url: String,
    issue_events_url:  String,
    issues_url:        String,
    keys_url:          String,
    labels_url:        String,
    languages_url:     String,
    merges_url:        String,
    milestones_url:    String,
    mirror_url:        Option<String>,
    notifications_url: String,
    pulls_url:         String,
    releases_url:      String,
    ssh_url:           String,
    stargazers_url:    String,
    statuses_url:      String,
    subscribers_url:   String,
    subscription_url:  String,
    svn_url:           String,
    tags_url:          String,
    teams_url:         String,
    trees_url:         String,
    homepage:          Option<String>,
    language:          Option<String>,
    forks_count:       u64,
    stargazers_count:  u64,
    watchers_count:    u64,
    size:              u64,
    default_branch:    String,
    open_issues_count: u64,
    has_issues:        bool,
    has_wiki:          bool,
    has_pages:         bool,
    has_downloads:     bool,
    pushed_at:         String,
    created_at:        String,
    updated_at:        String,
    permissions:       Permissions
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct User {
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Permissions {
    admin: bool,
    push:  bool,
    pull:  bool
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
pub fn get_users_username_starred(client: &mut Client, username: String, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Repo>, error::Error> {

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

    let mut response     = try!(client.get(url.as_str().to_string(), None));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)

}

pub fn get_users_starred(client: &mut Client, sort: Option<Sort>, direction: Option<Direction>) -> Result<Vec<Repo>, error::Error> {

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

    let mut response     = try!(client.get(url.as_str().to_string(), None));
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

    let mut response     = try!(client.get(url.as_str().to_string(), Some(header)));
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

    let mut response     = try!(client.get(url.as_str().to_string(), Some(header)));
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