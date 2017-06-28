// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Organizations
//!
//! Modules for subsections of Organizations in the Github
//! API docs and endpoints for Organizations.
//!
//! Reference: https://developer.github.com/v3/orgs/

use hyper::{Error as HyperError, Uri};
use serde_json;

use client::Client;
use error;
use utils;

/// Endpoints for Members.
pub mod members;

//TODO: Once out of preview
// pub mod outside_collaborators

/// Return type for Organizations endpoints.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Organization {
    pub login:               String,
    pub id:                  u64,
    pub url:                 String,
    pub repos_url:           String,
    pub events_url:          String,
    pub hooks_url:           Option<String>,
    pub issues_url:          Option<String>,
    pub members_url:         String,
    pub public_members_url:  String,
    pub avatar_url:          String,
    pub description:         Option<String>,
    pub name:                Option<String>,
    pub company:             Option<String>,
    pub blog:                Option<String>,
    pub location:            Option<String>,
    pub email:               Option<String>,
    pub public_repos:        Option<u64>,
    pub public_gists:        Option<u64>,
    pub followers:           Option<u64>,
    pub following:           Option<u64>,
    pub html_url:            Option<String>,
    pub created_at:          Option<String>,
    #[serde(rename = "type")]
    pub org_type:            Option<String>,
    pub total_private_repos: Option<u64>,
    pub owned_private_repos: Option<u64>,
    pub private_gists:       Option<u64>,
    pub disk_usage:          Option<u64>,
    pub collaborators:       Option<u64>,
    pub billing_email:       Option<String>,
    pub plan:                Option<Plan>
    //TODO: Once out of preview
    // pub default_repository_settings: Option<String>,
    // pub members_can_create_repositories: Option<String>
}

/// Sub-component fot Organization
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Plan {
    pub name:          String,
    pub space:         u64,
    pub private_repos: u64
}

///Input parameter for editing organizations.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrganizationEditParam {
    /// Billing email address.
    pub billing_email: String,
    /// Company name.
    pub company:       String,
    /// Publicly visible email address.
    pub email:         String,
    /// The location of the organization.
    pub location:      String,
    /// Shorthand name of the company.
    pub name:          String,
    /// Description of the company.
    pub description:   String
    //TODO: Once out of preview
    // pub default_repository_permission: String,
    // pub members_can_create_repositories: String
}


////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait OrganizationsExt {

    /// \[[Reference](https://developer.github.com/v3/orgs/#list-your-organizations)\]
    /// Returns the list of organizations for an
    /// authenticated user. `user` and `read:org` scopes
    /// are required to use this endpoint.
    /// ## Endpoint:
    /// GET /user/orgs
    fn get_user_orgs(&mut self) -> Result<Vec<Organization>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/#list-all-organizations)\]
    /// Returns the list of all organizations.
    /// ## Endpoint:
    /// GET /organizations
    /// ## Parameters:
    /// * `since`: Integer ID of the last organization you
    /// have seen
    fn get_organizations(&mut self, since: u64) -> Result<Vec<Organization>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/#list-user-organizations)\]
    /// Returns the list of a user's organizations.
    /// ## Endpoint:
    /// GET /users/:username/orgs
    /// ## Parameters:
    /// * `username`: Name of the user.
    fn get_users_username_orgs(&mut self, username: String) -> Result<Vec<Organization>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/#get-an-organization)\]
    /// Returns a single organizations.
    /// ## Endpoint:
    /// GET /orgs/:org
    /// ## Parameters:
    /// * `org`: Name of the oganization.
    fn get_orgs_org(&mut self, org: String) -> Result<Organization, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/#edit-an-organization)\]
    /// Edits an organization.
    /// ## Endpoint:
    /// PATCH /orgs/:org
    /// ## Parameters:
    /// * `org`: Name of the oganization.
    /// * `org_param`: Parameter for editing an
    /// organization.
    fn patch_orgs_org(&mut self, org: String, org_param: OrganizationEditParam) -> Result<Organization, error::Error>;
}

impl OrganizationsExt for Client {

    fn get_user_orgs(&mut self) -> Result<Vec<Organization>, error::Error>{
        utils::request_endpoint(self, "/user/orgs".to_string())
    }

    fn get_organizations(&mut self, since: u64) -> Result<Vec<Organization>, error::Error>{

        let mut url = match Uri::to_str(&format!("{}/organizations", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            query_pairs.append_pair("since", &format!("{}", since)[..]);
        }

        utils::request_endpoint(self, format!("/organizations?{}", url.query().unwrap()))
    }

    fn get_users_username_orgs(&mut self, username: String) -> Result<Vec<Organization>, error::Error>{
        utils::request_endpoint(self, format!("/users/{}/orgs", username))
    }

    fn get_orgs_org(&mut self, org: String) -> Result<Organization, error::Error>{
        utils::request_endpoint(self, format!("/orgs/{}", org))
    }

    fn patch_orgs_org(&mut self, org: String, org_param: OrganizationEditParam) -> Result<Organization, error::Error>{

        //Create body
        let body_data = try!(serde_json::to_string(&org_param).map_err(error::Error::Parsing));

        let mut response     = try!(self.patch_body(format!("/orgs/{}", org), None, body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }
}

//TODO: TESTS
