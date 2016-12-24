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
    pub starred_at: String,
    pub user:       User
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ListRepoStarTimeStamp {
    pub starred_at: String,
    pub repo:       Repo
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Repo {
    pub id:                u64,
    pub owner:             User,
    pub name:              String,
    pub full_name:         String,
    pub description:       Option<String>,
    pub private:           bool,
    pub fork:              bool,
    pub url:               String,
    pub html_url:          String,
    pub archive_url:       String,
    pub assignees_url:     String,
    pub blobs_url:         String,
    pub branches_url:      String,
    pub clone_url:         String,
    pub collaborators_url: String,
    pub comments_url:      String,
    pub commits_url:       String,
    pub compare_url:       String,
    pub contents_url:      String,
    pub contributors_url:  String,
    pub deployments_url:   String,
    pub downloads_url:     String,
    pub events_url:        String,
    pub forks_url:         String,
    pub git_commits_url:   String,
    pub git_refs_url:      String,
    pub git_tags_url:      String,
    pub git_url:           String,
    pub hooks_url:         String,
    pub issue_comment_url: String,
    pub issue_events_url:  String,
    pub issues_url:        String,
    pub keys_url:          String,
    pub labels_url:        String,
    pub languages_url:     String,
    pub merges_url:        String,
    pub milestones_url:    String,
    pub mirror_url:        Option<String>,
    pub notifications_url: String,
    pub pulls_url:         String,
    pub releases_url:      String,
    pub ssh_url:           String,
    pub stargazers_url:    String,
    pub statuses_url:      String,
    pub subscribers_url:   String,
    pub subscription_url:  String,
    pub svn_url:           String,
    pub tags_url:          String,
    pub teams_url:         String,
    pub trees_url:         String,
    pub homepage:          Option<String>,
    pub language:          Option<String>,
    pub forks_count:       u64,
    pub stargazers_count:  u64,
    pub watchers_count:    u64,
    pub size:              u64,
    pub default_branch:    String,
    pub open_issues_count: u64,
    pub has_issues:        bool,
    pub has_wiki:          bool,
    pub has_pages:         bool,
    pub has_downloads:     bool,
    pub pushed_at:         String,
    pub created_at:        String,
    pub updated_at:        String,
    pub permissions:       Permissions
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct User {
    pub login:               String,
    pub id:                  u64,
    pub avatar_url:          String,
    pub gravatar_id:         String,
    pub url:                 String,
    pub html_url:            String,
    pub followers_url:       String,
    pub following_url:       String,
    pub gists_url:           String,
    pub starred_url:         String,
    pub subscriptions_url:   String,
    pub organizations_url:   String,
    pub repos_url:           String,
    pub events_url:          String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type:           String,
    pub site_admin:          bool
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Permissions {
    pub admin: bool,
    pub push:  bool,
    pub pull:  bool
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