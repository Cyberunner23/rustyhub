// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

///Common structures found across many Github API responses
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Repository {
    pub id:                u64,
    pub name:              String,
    pub url:               String,
    pub owner:             Option<User>,
    pub full_name:         Option<String>,
    pub description:       Option<String>,
    pub private:           Option<bool>,
    pub fork:              Option<bool>,
    pub html_url:          Option<String>,
    pub archive_url:       Option<String>,
    pub assignees_url:     Option<String>,
    pub blobs_url:         Option<String>,
    pub branches_url:      Option<String>,
    pub clone_url:         Option<String>,
    pub collaborators_url: Option<String>,
    pub comments_url:      Option<String>,
    pub commits_url:       Option<String>,
    pub compare_url:       Option<String>,
    pub contents_url:      Option<String>,
    pub contributors_url:  Option<String>,
    pub deployments_url:   Option<String>,
    pub downloads_url:     Option<String>,
    pub events_url:        Option<String>,
    pub forks_url:         Option<String>,
    pub git_commits_url:   Option<String>,
    pub git_refs_url:      Option<String>,
    pub git_tags_url:      Option<String>,
    pub git_url:           Option<String>,
    pub hooks_url:         Option<String>,
    pub issue_comment_url: Option<String>,
    pub issue_events_url:  Option<String>,
    pub issues_url:        Option<String>,
    pub keys_url:          Option<String>,
    pub labels_url:        Option<String>,
    pub languages_url:     Option<String>,
    pub merges_url:        Option<String>,
    pub milestones_url:    Option<String>,
    pub mirror_url:        Option<String>,
    pub notifications_url: Option<String>,
    pub pulls_url:         Option<String>,
    pub releases_url:      Option<String>,
    pub ssh_url:           Option<String>,
    pub stargazers_url:    Option<String>,
    pub statuses_url:      Option<String>,
    pub subscribers_url:   Option<String>,
    pub subscription_url:  Option<String>,
    pub svn_url:           Option<String>,
    pub tags_url:          Option<String>,
    pub teams_url:         Option<String>,
    pub trees_url:         Option<String>,
    pub homepage:          Option<String>,
    pub language:          Option<String>,
    pub forks_count:       Option<u64>,
    pub stargazers_count:  Option<u64>,
    pub watchers_count:    Option<u64>,
    pub size:              Option<u64>,
    pub default_branch:    Option<String>,
    pub open_issues_count: Option<u64>,
    pub has_issues:        Option<bool>,
    pub has_wiki:          Option<bool>,
    pub has_pages:         Option<bool>,
    pub has_downloads:     Option<bool>,
    pub pushed_at:         Option<String>,
    pub created_at:        Option<String>,
    pub updated_at:        Option<String>,
    pub permissions:       Option<Permissions>
}

/// Sub-component for the Repository and the response type
/// some endpoints
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    pub login:               String,
    pub id:                  u64,
    pub avatar_url:          String,
    pub gravatar_id:         String,
    pub url:                 String,
    pub html_url:            Option<String>,
    pub followers_url:       Option<String>,
    pub following_url:       Option<String>,
    pub gists_url:           Option<String>,
    pub starred_url:         Option<String>,
    pub subscriptions_url:   Option<String>,
    pub organizations_url:   Option<String>,
    pub repos_url:           Option<String>,
    pub events_url:          Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub user_type:           Option<String>,
    pub site_admin:          Option<bool>
}

/// Sub-component of the Repository
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Permissions {
    pub admin: bool,
    pub push:  bool,
    pub pull:  bool
}

///Common structures found across some Github API responses.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Comment {
    pub id:         u64,
    pub url:        String,
    pub html_url:   Option<String>,
    pub body:       String,
    pub user:       User,
    pub created_at: String,
    pub updated_at: String
}

//TODO: Tests
