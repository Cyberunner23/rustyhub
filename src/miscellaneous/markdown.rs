// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use hyper::header::{Accept, ContentType, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde_json;

use client::Client;
use error;

///Reference: https://developer.github.com/v3/markdown/

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct MarkdownRequest {
    text:    String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode:    Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>
}

pub enum MarkdownRawMIME {
    TextPlain,
    TextXMarkdown
}


////////////////////////////////////////////////////////////
//                       Functions                        //
////////////////////////////////////////////////////////////

///Reference: https://developer.github.com/v3/markdown/#render-an-arbitrary-markdown-document
pub fn post_markdown(client: &mut Client, request_body: &MarkdownRequest) -> Result<String, error::Error> {

    //Set Accept
    let mut header = client.get_default_header();
    header.remove::<Accept>();
    header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Html, vec![]))]));

    //Create body
    let body_data = try!(serde_json::to_string(&request_body).map_err(error::Error::Parsing));

    let mut response     = try!(client.post_body("/markdown".to_string(), Some(header), body_data));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

///Reference: https://developer.github.com/v3/markdown/#render-a-markdown-document-in-raw-mode
pub fn post_markdown_raw(client: &mut Client, request_body: &MarkdownRequest, mime: MarkdownRawMIME) -> Result<String, error::Error> {

    //Edit Accept and set Content-Type
    let mut header = client.get_default_header();
    header.remove::<Accept>();
    header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Html, vec![]))]));

    match mime {
        MarkdownRawMIME::TextPlain     => header.set(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![]))),
        MarkdownRawMIME::TextXMarkdown => header.set(ContentType(Mime(TopLevel::Text, SubLevel::Ext("x-markdown".to_string()), vec![]))),
    }

    //Create body
    let body_data = try!(serde_json::to_string(&request_body).map_err(error::Error::Parsing));

    let mut response     = try!(client.post_body("/markdown/raw".to_string(), Some(header), body_data));
    let     response_str = try!(Client::response_to_string(&mut response));
    serde_json::from_str(&response_str[..]).map_err(error::Error::Parsing)
}

//TODO: TESTS
