// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use hyper::header::{Accept, ContentType, Headers, qitem};
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

pub fn post_markdown(client: &mut Client, request_body: &MarkdownRequest) -> Result<String, error::Error> {

    //Set Content-Type
    let mut header: Headers = client.get_default_header();
    header.remove::<Accept>();
    header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Html, vec![]))]));

    //Create body
    let body_data = match serde_json::to_string(&request_body) {
        Ok(body_data) => body_data,
        Err(err)      => return Err(error::Error::Parsing(err))
    };

    Ok(try!(Client::response_to_string(&mut try!(client.post_body("/markdown".to_string(), Some(header), body_data)))))
}

pub fn post_markdown_raw(client: &mut Client, request_body: &MarkdownRequest, mime: MarkdownRawMIME) -> Result<String, error::Error> {

    //Create body
    let body_data = match serde_json::to_string(&request_body) {
        Ok(body_data) => body_data,
        Err(err)      => return Err(error::Error::Parsing(err))
    };

    //Edit Accept and set Content-Type
    let mut header: Headers = client.get_default_header();
    header.remove::<Accept>();
    header.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Html, vec![]))]));

    match mime {
        MarkdownRawMIME::TextPlain     => header.set(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![]))),
        MarkdownRawMIME::TextXMarkdown => header.set(ContentType(Mime(TopLevel::Text, SubLevel::Ext("x-markdown".to_string()), vec![]))),
    }

    Ok(try!(Client::response_to_string(&mut try!(client.post_body("/markdown/raw".to_string(), Some(header), body_data)))))
}