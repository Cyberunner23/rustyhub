// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Markdown
//!
//! These are the responses and API call functions related
//! to the markdown endpoints of the API.
//!
//! Reference: https://developer.github.com/v3/markdown/

use hyper::header::{Accept, ContentType, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde_json;

use client::Client;
use error;

/// Input parameters for the markdown endpoints.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct MarkdownRequest {
    /// Markdown data
    pub text:    String,
    /// * `"markdown"` to render a document as plain
    /// Markdown, just like README files are rendered.
    /// * `"gfm"` to render a document as user-content,
    /// e.g. like user comments or issues are rendered.
    /// In GFM mode, hard line breaks are always taken
    /// into account, and issue and user mentions are
    /// linked accordingly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode:    Option<String>,
    /// The repository context. Only taken into account
    /// when rendering as `"gfm"`, ex.
    /// "cyberunner23/rustyhub"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>
}

/// Requested mimetype of the rendered text for the raw
/// variant.
pub enum MarkdownRawMIME {
    TextPlain,
    TextXMarkdown
}


////////////////////////////////////////////////////////////
//                    Extension Trait                     //
////////////////////////////////////////////////////////////

pub trait MarkdownExt {

    /// \[[Reference](https://developer.github.com/v3/markdown/#render-an-arbitrary-markdown-document)\]
    /// Returns the rendered markdown as html.
    /// ## Endpoint:
    /// POST /markdown
    /// ## Parameters:
    /// * `request_body`: Body of the request, see MarkdownRequest.
    fn post_markdown(&mut self, request_body: &MarkdownRequest) -> Result<String, error::Error>;

    /// \[[Reference](https://developer.github.com/v3/markdown/#render-a-markdown-document-in-raw-mode)\]
    /// Returns the rendered markdown as html.
    /// ## Endpoint:
    /// POST /markdown
    /// ## Parameters:
    /// * `request_body`: Body of the request, see MarkdownRequest.
    /// * `mime`:
    fn post_markdown_raw(&mut self, request_body: &MarkdownRequest, mime: MarkdownRawMIME) -> Result<String, error::Error>;

}

impl MarkdownExt for Client {

    fn post_markdown(&mut self, request_body: &MarkdownRequest) -> Result<String, error::Error> {

        //Set Accept
        let mut header = self.get_default_headers();
        header.remove::<Accept>();
        header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Html, vec![]))]));

        //Create body
        let body_data = try!(serde_json::to_string(&request_body).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body("/markdown".to_string(), Some(header), body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }

    fn post_markdown_raw(&mut self, request_body: &MarkdownRequest, mime: MarkdownRawMIME) -> Result<String, error::Error> {

        //Edit Accept and set Content-Type
        let mut header = self.get_default_headers();
        header.remove::<Accept>();
        header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Html, vec![]))]));

        match mime {
            MarkdownRawMIME::TextPlain     => header.set(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![]))),
            MarkdownRawMIME::TextXMarkdown => header.set(ContentType(Mime(TopLevel::Text, SubLevel::Ext("x-markdown".to_string()), vec![]))),
        }

        //Create body
        let body_data = try!(serde_json::to_string(&request_body).map_err(error::Error::Parsing));

        let mut response     = try!(self.post_body("/markdown/raw".to_string(), Some(header), body_data));
        let     response_str = try!(Client::response_to_string(&mut response));
        serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
    }
}

//TODO: TESTS
