
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use serde_json::Value;
use utils::*;


///Reference: https://developer.github.com/v3/activity/events/

/// The response of most event requests.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    event_type: String,
    public:     bool,
    //payload:    Payload,
    repo:       Repository,
    actor:      Actor,
    org:        Organization,
    created_at: String,
    id:         u64
}

impl Event{

    pub fn new() -> Event {
        Event {
            event_type: String::new,
            public:     true,
            //payload:    Payload,
            repo:       Repository::new,
            actor:      Actor::new(),
            org:        Organization::new,
            created_at: String::new(),
            id:         0
        }
    }

    //Finish this
    pub fn from_val(value: Value) -> Result<Event, Error> {
        let mut object = try!(extract_object(value));
        Event {
            event_type: try!(extract_string(object, "event_type")),
            public:     try!(extract_bool(object, "public")),
            //payload:
            repo:       try!(Repository::from_val(try!(extract_map_val(object, "repo")))),
            actor:      try!(Actor::from_val(try!(extract_map_val(object, "actor")))),
            org:        try!(Organization::from_val(try!(extract_map_val(object, "org")))),
            created_at: try!(extract_string(object, "created_at")),
            id:         try!(extract_u64(object, "id"))
        }
    }

}


//TODO: Implement new, from_json/value
/*#[derive(Clone, Debug, PartialEq)]
pub enum Payload {

}*/


#[derive(Clone, Debug, PartialEq)]
pub struct Repository {
    id:   u64,
    name: String,
    url:  String
}

impl Repository {

    pub fn new() -> Repository {
        Repository {
            id:   0,
            name: String::new(),
            url:  String::new()
        }
    }

    pub fn from_val(value: Value) -> Result<Repository, Error> {
        let mut object = try!(extract_object(value));
        Repository {
            id:   try!(extract_u64(object, "id")),
            name: try!(extract_string(object, "name")),
            url:  try!(extract_string(object, "url")),
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Actor {
    id:          u64,
    login:       String,
    gravatar_id: String,
    avatar_url:  String,
    url:         String
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            id:          0,
            login:       String::new(),
            gravatar_id: String::new(),
            avatar_url:  String::new(),
            url:         String::new()
        }
    }

    pub fn from_val(value: Value) -> Result<Actor, Error> {
        let mut object = try!(extract_object(value));
        Actor {
            id:          try!(extract_u64(object, "id")),
            login:       try!(extract_string(object, "login")),
            gravatar_id: try!(extract_string(object, "gravatar_url")),
            avatar_url:  try!(extract_string(object, "avatar_url")),
            url:         try!(extract_string(object, "url"))
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Organization {
    id:          u64,
    login:       String,
    gravatar_id: String,
    avatar_url:  String,
    url:         String,
}

impl Organization {
    pub fn new() -> Organization {
        Organization {
            id:          0,
            login:       String::new(),
            gravatar_id: String::new(),
            avatar_url:  String::new(),
            url:         String::new(),
        }
    }

    pub fn from_val(value: Value) -> Result<Organization, Error> {
        let mut object = try!(extract_object(value));
        Organization {
            id:          try!(extract_u64(object, "id")),
            login:       try!(extract_string(object, "login")),
            gravatar_id: try!(extract_string(object, "gravatar_url")),
            avatar_url:  try!(extract_string(object, "avatar_url")),
            url:         try!(extract_string(object, "url"))
        }
    }
}
