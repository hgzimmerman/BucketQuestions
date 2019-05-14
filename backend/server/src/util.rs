//! Common utilities
use crate::error::Error;
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

/// A path filter that specifies the remaining segment(s) to match upon.
#[allow(dead_code)]
pub fn terminal_path(path: &'static str) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::path(path).and(warp::path::end())
}

const KILOBYTE: u64 = 1024;
/// Extracts the body of a request after stipulating that it has a reasonable size in kilobytes.
///
/// # Arguments
/// * kb_limit - The maximum number of kilobytes, over which the request will be rejected.
/// This is done to limit abusively sized requests.
pub fn json_body_filter<T>(kb_limit: u64) -> impl Filter<Extract = (T,), Error = Rejection> + Copy
where
    T: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    warp::body::content_length_limit(KILOBYTE * kb_limit).and(warp::body::json())
}

#[allow(dead_code)]
/// Util function that makes replying easier.
pub fn json_convert<T, U>(source: T) -> impl Reply
where
    T: Into<U>,
    U: Serialize,
{
    let target: U = source.into();
    warp::reply::json(&target)
}

/// Converts a serializable type to a JSON reply.
pub fn json<T>(source: T) -> impl Reply
where
    T: Serialize,
{
    warp::reply::json(&source)
}

/// Either converts a result to json or creates a rejection from the error.
pub fn json_or_reject<T, E>(source: Result<T, E>) -> Result<impl Reply, Rejection>
where
    T: Serialize,
    E: Into<Error>,
{
    source.map(json).map_err(|e| e.into().reject())
}

#[allow(dead_code)]
pub fn reject<T, E>(source: Result<T, E>) -> Result<T, Rejection>
where
    E: Into<Error>,
{
    source.map_err(|e| e.into().reject())
}

/// Converts a vector of T to a vector of U then converts the U vector to a JSON reply.
#[allow(dead_code)]
pub fn many_json_converts<T, U>(source: impl IntoIterator<Item = T>) -> impl Reply
where
    U: From<T>,
    U: Serialize,
{
    let target: Vec<U> = source.into_iter().map(U::from).collect();
    warp::reply::json(&target)
}

#[cfg(test)]
pub mod test_util {
    use bytes::Bytes;
    use hyper::Response;
    use serde::Deserialize;
    use serde_json::from_str;
    use std::ops::Deref;

    /// Used in testing, this function will try to deserialize a response generated from a typical
    /// warp::testing::request() invocation.
    pub fn deserialize<'de, T: Deserialize<'de>>(response: &'de Response<Bytes>) -> T {
        let body = response.body();
        let bytes: &[u8] = body.deref();
        let body_string = std::str::from_utf8(bytes).expect("valid utf8 string");
        eprintln!("Body string: {}", body_string);
        from_str::<T>(body_string).expect("Should be able to deserialize body")
    }

    #[allow(unused)]
    pub fn deserialize_string(response: Response<Bytes>) -> String {
        let body = response.into_body();
        let bytes: &[u8] = body.deref();
        let body_string = std::str::from_utf8(bytes).expect("valid utf8 string");
        String::from(body_string)
    }
}
