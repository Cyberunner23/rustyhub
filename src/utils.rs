
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


use std::collections::BTreeMap;
use serde_json::Value;
use error::{Error, DecodeError};

///Small utility to convert an object Value to an object map
pub fn extract_object(value: Value) -> Result<BTreeMap<String, Value>, Error> {
    match value {
        Value::Object(obj) => Ok(obj),
        value              => Err(Error::Decoding(DecodeError::InvalidObject(value)))
    }
}

///Small utility to extract a named object map from an object map
/*pub fn extract_named_object(object: &mut BTreeMap<String, Value>, field: &'static str) -> Result<BTreeMap<String, Value>, Error> {
    let value = try!(extract_map_val(object, field));
    match value {
        Value::Object(obj) => Ok(obj),
        value              => Err(Error::Decoding(DecodeError::InvalidObject(value)))
    }
}*/


///Small utility to extract a u64 from a Value
pub fn extract_u64(object: &mut BTreeMap<String, Value>, field: &'static str) -> Result<u64, Error> {
    let value = try!(extract_map_val(object, field));
    Ok(try!(value.as_u64().ok_or(Error::Decoding(DecodeError::InvalidU64(value)))))
}


///Small utility to extract a string from a Value
pub fn extract_string(object: &mut BTreeMap<String, Value>, field: &'static str) -> Result<String, Error> {
    let value = try!(extract_map_val(object, field));
    Ok(try!(value.as_str().ok_or(Error::Decoding(DecodeError::InvalidString(value.clone())))).to_string())
}

///Small utility to extract a bool from a Value
pub fn extract_bool(object: &mut BTreeMap<String, Value>, field: &'static str) -> Result<bool, Error> {
    let value = try!(extract_map_val(object, field));
    Ok(try!(value.as_bool().ok_or(Error::Decoding(DecodeError::InvalidBool(value.clone())))))
}


/// Small utility to extract a JSON Value from an object map
pub fn extract_map_val(object: &mut BTreeMap<String, Value>, field: &'static str) -> Result<Value, Error> {
    object.remove(field).ok_or(Error::Decoding(DecodeError::FieldNotFound(field.clone(), Value::Object(object.clone()))))
}