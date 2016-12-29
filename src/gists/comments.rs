// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Comments
//!
//! These are the responses and API call functions related
//! to the comments endpoints of the API.
//!
//! Reference: https://developer.github.com/v3_gists_comments/

use hyper::{Error as HyperError, Url};
use serde_json;

use client::Client;
use common::Comment;
use issues::{Direction, Sort};
use error;
use utils;

///Internal parameter
#[derive(Clone, Debug, PartialEq, Serialize)]
struct CommentBody {
    body: String
}

////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait CommentsExt {

        /// \[[Reference](https://developer.github.com/v3/gists/#list-a-users-gists)\]
    /// Returns the list of comments on a gist.
    /// ## Endpoint:
    /// GET /gists/:gist_id/comments
    /// ## Parameters
    /// * `gist_id`: ID of the gist.
    fn get_gists_gist_id_comments(&mut self, gist_id: u64) -> Result<Vec<Comment>, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/comments/#get-a-single-comment)\]
    /// Returns a single comment on a gist.
    /// ## Endpoint:
    /// GET /gists/:gist_id/comments/:id
    /// ## Parameters
    /// * `gist_id`: ID of the gist.
    /// * `id`: ID of the comment.
    fn get_gists_gist_id_comments_id(&mut self, gist_id: u64, id: u64) -> Result<Comment, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/comments/#create-a-comment)\]
    /// Creates a comment.
    /// ## Endpoint:
    /// POST /gists/:gist_id/comments
    /// ## Parameters
    /// * `gist_id`: ID of the gist.
    /// * `body`: Contents of the comment.
    fn post_gists_gist_id_comments(&mut self, gist_id: u64, body: String) -> Result<Comment, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/comments/#edit-a-comment)\]
    /// Edits a comment.
    /// ## Endpoint:
    /// PATCH /gists/:gist_id/comments/:id
    /// ## Parameters
    /// * `gist_id`: ID of the gist.
    /// * `id`: ID of the comment.
    /// * `body`: Contents of the comment.
    fn patch_gists_gist_id_comments_id(&mut self, gist_id: u64, id: u64, body: String) -> Result<Comment, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/gists/comments/#delete-a-comment)\]
    /// Deletes a comment.
    /// ## Endpoint:
    /// DELETE /gists/:gist_id/comments/:id
    /// ## Parameters
    /// * `gist_id`: ID of the gist.
    /// * `id`: ID of the comment.
    fn delete_gists_gist_id_comments_id(&mut self, gist_id: u64, id: u64) -> Result<(), error::Error>;
}

impl CommentsExt for Client {

    fn get_gists_gist_id_comments(&mut self, gist_id: u64) -> Result<Vec<Comment>, error::Error>{
        utils::request_endpoint(self, format!("/gists/{}/comments", gist_id))
    }

    fn get_gists_gist_id_comments_id(&mut self, gist_id: u64, id: u64) -> Result<Comment, error::Error>{
        utils::request_endpoint(self, format!("/gists/{}/comments/{}", gist_id, id))
    }

    fn post_gists_gist_id_comments(&mut self, gist_id: u64, body: String) -> Result<Comment, error::Error>{

        //Serialize body
        let comment_body = try!(serde_json::to_string(&CommentBody{
            body: body
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body(format!("/gists/{}/comments", gist_id), None, comment_body));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn patch_gists_gist_id_comments_id(&mut self, gist_id: u64, id: u64, body: String) -> Result<Comment, error::Error>{

        //Serialize body
        let comment_body = try!(serde_json::to_string(&CommentBody{
            body: body
        }).map_err(error::Error::Parsing));

        let mut response     = try!(self.patch_body(format!("/gists/{}/comments", gist_id), None, comment_body));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn delete_gists_gist_id_comments_id(&mut self, gist_id: u64, id: u64) -> Result<(), error::Error>{
        match self.delete(format!("/gists/{}/comments/{}", gist_id, id), None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(err)
        }
    }
}

//TODO: Handle Custom media types
//TODO: TESTS
