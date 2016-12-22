
// Copyright 2016 Alex Frappier Lachapelle
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate serde_json;

/// Represents all possible errors than can occur while using rustyhub.
//TODO: Implement other relevant error types
pub enum Error {
    /// Error returned by serde when parsing JSON data from a string.
    Parsing(serde_json::Error),
    /// Error returned when decoding the JSON data.
    Decoding(DecodeError)
}


/// Represents errors that can occur when decoding JSON data.
/// Each error type includes the offending JSON value.
///
/// # Unnamed vs Named
///
/// A JSON field can be named or unnamed.
///
/// Unnamed fields are JSON typed used as the root of the data.
/// Example with an array of events:
///
/// ```
/// [
///     {
///         Event...
///     },
///     {
///         Event...
///     },
///     ...
/// ]
/// ```
///
/// Named fields of course have a name to them (sometimes called a key).
/// Example:
///
/// ```
/// {
///     "hooks": [
///         "issue_comment",
///         "pull_request_review_comment",
///         ...
///     ],
///     ...
/// }
/// ```
///
/// "Named" variants of each error includes the name/key in order to better
/// identify the location of the error.
///
/// Note: "Named" variants are only present for objects and arrays as they are the only types witn an "Unnamed" variant. All other types have a name.
///
///
pub enum DecodeError {
    /// A decoding error describing the absence of a JSON field.
    FieldNotFound(&'static str, serde_json::Value),
    /// A decoding error describing an invalid Array.
    InvalidArray(serde_json::Value),
    /// A decoding error describing an invalid named Array.
    InvalidNamedArray(&'static str, serde_json::Value),
    /// A decoding error describing an invalid Object.
    InvalidObject(serde_json::Value),
    /// A decoding error describing an invalid named Object.
    InvalidNamedObject(&'static str, serde_json::Value),
    /// A decoding error describing an invalid u64.
    InvalidU64(serde_json::Value),
    /// A decoding error describing an invalid string.
    InvalidString(serde_json::Value),
    /// A decoding error describing an invalid bool.
    InvalidBool(serde_json::Value),

}
