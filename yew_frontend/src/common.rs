use serde::Serialize;
use serde::de::DeserializeOwned;
use std::future::Future;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};

// TODO add remaining HTTP methods.
pub enum MethodData<'a, T> {
    Get,
    Delete,
    Post(&'a T),
    Put(&'a T)
}

impl <'a, T> MethodData<'a, T> {
    pub fn method_name(&self) -> &'static str {
        match self {
            MethodData::Get => "GET",
            MethodData::Delete => "DELETE",
            MethodData::Post(_) => "POST",
            MethodData::Put(_) => "PUT"
        }
    }


}

impl <'a, T: Serialize> MethodData<'a, T> {
    pub fn as_body(&self) -> Result<Option<JsValue>, FetchError> {
        let body: Option<String> = match self {
            MethodData::Get
            | MethodData::Delete => None,
            MethodData::Put(data)
            | MethodData::Post(data) => {
                let body = serde_json::to_string(data)
                    .map_err(|_| FetchError::CouldNotSerializeRequestBody)?;
                Some(body)
            }
        };

        let body = body
            .map(|data| JsValue::from_str(data.as_str()));
        Ok(body)
    }
}

// TODO, these can contain much more data.
pub enum FetchError {
    DeserializeError,
    TextNotAvailable,
    CouldNotCreateFetchFuture,
    CouldNotCreateRequest,
    CouldNotSerializeRequestBody
}

pub trait FetchRequest {
    type UrlProvider;
    type RequestData: Serialize;
    type ResponseType: DeserializeOwned;

    fn url(&self) -> String;

    fn method(&self) -> MethodData<Self::RequestData>;

    fn headers(&self) -> Vec<(String, String)>;

}

pub async fn fetch_resource<T: FetchRequest>(request: &T) -> Result<T::ResponseType, FetchError> {
    let method = request.method();

    // configure options for the request
    let mut opts = RequestInit::new();
    opts.method(method.method_name());
    opts.body(method.as_body()?.as_ref());

    opts.mode(RequestMode::Cors); // TODO make a thing for this

    // Create the request
    let request = Request::new_with_str_and_init(
        &request.url(),
        &opts,
    )
        .map_err(|_| FetchError::CouldNotCreateRequest)?;


    let window: Window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| FetchError::CouldNotCreateFetchFuture)?;
    debug_assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text().map_err(|_| FetchError::TextNotAvailable)?)
        .await
        .map_err(|_| FetchError::TextNotAvailable)?;

    let text_string = text.as_string().unwrap();

    let deserialized = serde_json::from_str(&text_string)
        .map_err(|_e| FetchError::DeserializeError)?;

    Ok(deserialized)
}
