// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

///! # Members
///!
///! These are the responses and API call functions related
///! to the members endpoints of the API.
///!

use hyper::{Error as HyperError, Uri};
use serde_json;

use client::Client;
use common::User;
use error;
use organizations::Organization;
use utils;

/// Parameter to get members.
/// Filter members returned in the list.
pub enum Filter {
    /// Members without two-factor authentication enabled.
    /// Available for organization owners.
    TwoFADisabled,
    /// All members the authenticated user can see.
    All
}

impl Filter {
    fn to_str(&self) -> String {
        match self {
            &Filter::TwoFADisabled => "2fa_disabled".to_string(),
            &Filter::All           => "all".to_string(),
        }
    }
}

/// Parameter to get members list.
/// Filter members returned by their role.
pub enum RoleMemberList {
    /// All members of the organization, regardless of role.
    All,
    /// Organization owners.
    Admin,
    /// Non-owner organization members.
    Member
}

impl RoleMemberList {
    fn to_str(&self) -> String {
        match self {
            &RoleMemberList::All=> "2fa_disabled".to_string(),
            &RoleMemberList::Admin=> "all".to_string(),
            &RoleMemberList::Member=> "all".to_string(),
        }
    }
}

/// Parameter to get members list.
/// Filter members returned by their role.
pub enum RoleUpdateMembership {
    /// Organization owners.
    Admin,
    /// Non-owner organization members.
    Member
}

impl RoleUpdateMembership {
    fn to_str(&self) -> String {
        match self {
            &RoleUpdateMembership::Admin=> "all".to_string(),
            &RoleUpdateMembership::Member=> "all".to_string(),
        }
    }
}

/// Parameter to list Organization memberships.
/// FIndicates the state of the memberships to return.
pub enum State {
    /// User's membership is active.
    Active,
    /// User's membership is pending approval.
    Pending
}

impl State {
    fn to_str(&self) -> String {
        match self {
            &State::Active  => "active".to_string(),
            &State::Pending => "pending".to_string(),
        }
    }
}

/// Return type for getting an organization membership.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct OrgMembership {
    pub url:              String,
    pub state:            String,
    pub role:             String,
    pub organization_url: String,
    pub organization:     Organization,
    pub user:             User
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait MembersExt {

    /// \[[Reference](https://developer.github.com/v3/orgs/members#members-list)\]
    /// Returns a list of all users who are members of an
    /// organization. If the authenticated user is also a
    /// member of this organization then both concealed and
    /// public members will be returned.
    /// ## Endpoint:
    /// GET /orgs/:org/members
    /// ## Parameters:
    /// * `org`: Name of the organization
    /// * `filter`: Default: `Filter::All` Filter members
    /// returned in the list.
    /// * `role`: Default: `RoleMemberList::All` Filter
    /// members returned by their role.
    fn get_orgs_org_members(&mut self, org: String, filter: Option<Filter>, role: Option<RoleMemberList>) -> Result<Vec<User>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#check-membership)\]
    /// Checks if a user is, publicly or privately, a
    /// member of the organization.
    /// ## Endpoint:
    /// GET /orgs/:org/members/:username
    /// ## Parameters:
    /// * `org`: Name of the organization
    /// * `username`: Name of the user
    //TODO: Handle 302
    fn get_orgs_org_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#remove-a-member)\]
    /// Removes a member from the organization.
    /// ## Endpoint:
    /// DELETE /orgs/:org/members/:username
    /// ## Parameters:
    /// * `org`: Name of the organization
    /// * `username`: Name of the user
    fn delete_orgs_org_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#public-members-list)\]
    /// Returns a list of an organizations public members.
    /// ## Endpoint:
    /// GET /orgs/:org/public_members
    /// ## Parameters:
    /// * `org`: Name of the organization
    fn get_orgs_org_public_members(&mut self, org: String) -> Result<Vec<User>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#check-public-membership)\]
    /// Checks if a member is publicly displayed as a
    /// member.
    /// ## Endpoint:
    /// GET /orgs/:org/public_members/:username
    /// ## Parameters:
    /// * `org`: Name of the organization
    /// * `username`: Name of the user
    fn get_orgs_org_public_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#publicize-a-users-membership)\]
    /// Publicizes a user's membership
    /// ## Endpoint:
    /// PUT /orgs/:org/public_members/:username
    /// ## Parameters:
    /// * `org`: Name of the organization
    /// * `username`: Name of the user
    fn put_orgs_org_public_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#conceal-a-users-membership)\]
    /// Conceals a user's membership
    /// ## Endpoint:
    /// DELETE /orgs/:org/public_members/:username
    /// ## Parameters:
    /// * `org`: Name of the organization
    /// * `username`: Name of the user
    fn delete_orgs_org_public_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#get-organization-membership)\]
    /// Returns the user's membership.
    /// ## Endpoint:
    /// GET /orgs/:org/memberships/:username
    /// ## Parameters:
    /// * `org`: Name of the organization
    /// * `username`: Name of the user
    fn get_orgs_org_memberships_username(&mut self, org: String, username: String) -> Result<OrgMembership, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#add-or-update-organization-membership)\]
    /// Adds/Updated an organization membership.
    /// ## Endpoint:
    /// PUT /orgs/:org/memberships/:username
    /// ## Parameters:
    /// * `org`: Name of the organization
    /// * `username`: Name of the user
    /// * `role`: Default: `RoleUpdateMembership::Member`
    /// Filter members returned by their role.
    fn put_orgs_org_memberships_username(&mut self, org: String, username: String, role: Option<RoleUpdateMembership>) -> Result<OrgMembership, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#remove-organization-membership)\]
    /// Removes an organization membership.
    /// ## Endpoint:
    /// DELETE /orgs/:org/memberships/:username
    /// ## Parameters:
    /// * `org`: Name of the organization
    /// * `username`: Name of the user
    fn delete_orgs_org_memberships_username(&mut self, org: String, username: String) -> Result<(), error::Error>;

    //TODO: Once out pf preview.
    /*/// \[[Reference](https://developer.github.com/v3/orgs/members#list-pending-organization-invitations)\]
    /// ** Description **
    /// ## Endpoint:
    /// GET /orgs/:org/invitations
    /// ## Parameters:
    fn get_orgs_org_invitations(&mut self) -> Result<, error::Error>;*/

    /// \[[Reference](https://developer.github.com/v3/orgs/members#list-your-organization-memberships)\]
    /// Lists an authenticated user's memberships.
    /// ## Endpoint:
    /// GET /user/memberships/orgs
    /// ## Parameters:
    /// * `state`: Default: Active and Pending. Indicates
    /// the state of the memberships to return.
    fn get_user_memberships_orgs(&mut self, state: Option<State>) -> Result<Vec<OrgMembership>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#get-your-organization-membership)\]
    /// Gets an authenticates user's membership.
    /// ## Endpoint:
    /// GET /user/memberships/orgs/:org
    /// ## Parameters:
    /// * `org`: Name of the organization
    fn get_user_memberships_orgs_org(&mut self, org: String) -> Result<OrgMembership, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/orgs/members#edit-your-organization-membership)\]
    /// Edits an organization membership.
    /// ## Endpoint:
    /// PATCH /user/memberships/orgs/:org
    /// ## Parameters:
    /// * `org`: Name of the organization
    fn patch_user_memberships_orgs_org(&mut self, org: String) -> Result<OrgMembership, error::Error>;
}

impl MembersExt for Client {

    fn get_orgs_org_members(&mut self, org: String, filter: Option<Filter>, role: Option<RoleMemberList>) -> Result<Vec<User>, error::Error>{

        let mut url = match Uri::to_str(&format!("{}/orgs/{}/members", self.api_url, org)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = filter {
                query_pairs.append_pair("filter", &param.to_str());
            }

            if let Some(param) = role {
                query_pairs.append_pair("role", &param.to_str());
            }
        }

        utils::request_endpoint(self, format!("/orgs/{}/members?{}", org, url.query().unwrap()))
    }

    fn get_orgs_org_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>{
        match self.get(format!("/orgs/{}/members/{}", org, username), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn delete_orgs_org_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>{
        match self.delete(format!("/orgs/{}/members/{}", org, username), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn get_orgs_org_public_members(&mut self, org: String) -> Result<Vec<User>, error::Error>{
        utils::request_endpoint(self, format!("/orgs/{}/public_members", org))
    }

    fn get_orgs_org_public_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>{
        match self.get(format!("/orgs/{}/public_members/{}", org, username), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    //TODO: Set content length to 0
    fn put_orgs_org_public_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>{
        match self.put(format!("/orgs/{}/public_members/{}", org, username), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn delete_orgs_org_public_members_username(&mut self, org: String, username: String) -> Result<(), error::Error>{
        match self.delete(format!("/orgs/{}/public_members/{}", org, username), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn get_orgs_org_memberships_username(&mut self, org: String, username: String) -> Result<OrgMembership, error::Error>{
        utils::request_endpoint(self, format!("/orgs/{}/memberships/{}", org, username))
    }

    fn put_orgs_org_memberships_username(&mut self, org: String, username: String, role: Option<RoleUpdateMembership>) -> Result<OrgMembership, error::Error>{

        let mut url = match Uri::to_str(&format!("{}/orgs/{}/memberships/{}", self.api_url, org, username)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = role {
                query_pairs.append_pair("role", &param.to_str());
            }
        }

        let mut response     = try!(self.put(format!("/orgs/{}/memberships/{}?{}", org, username, url.query().unwrap()), None));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn delete_orgs_org_memberships_username(&mut self, org: String, username: String) -> Result<(), error::Error>{
        match self.delete(format!("/orgs/{}/memberships/{}", org, username), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn get_user_memberships_orgs(&mut self, state: Option<State>) -> Result<Vec<OrgMembership>, error::Error>{

        let mut url = match Uri::to_str(&format!("{}/user/memberships/orgs", self.api_url)[..]) {
            Ok(url)  => url,
            Err(err) => return Err(error::Error::HTTP(HyperError::Uri(err)))
        };

        //Limits the scope of the mutable borrow
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();

            if let Some(param) = state {
                query_pairs.append_pair("state", &param.to_str());
            }
        }

        utils::request_endpoint(self, format!("/user/memberships/orgs?{}", url.query().unwrap()))
    }

    fn get_user_memberships_orgs_org(&mut self, org: String) -> Result<OrgMembership, error::Error>{
        utils::request_endpoint(self, format!("/user/memberships/orgs/{}", org))
    }

    fn patch_user_memberships_orgs_org(&mut self, org: String) -> Result<OrgMembership, error::Error>{
        let mut response     = try!(self.patch_body(format!("/user/memberships/orgs/{}", org), None, "{\"state\":\"active\"}".to_string()));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }
}

//TODO: TESTS
