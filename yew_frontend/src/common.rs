use serde::Serialize;
use serde::de::DeserializeOwned;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use yewtil::NeqAssign;


#[derive(Clone, PartialEq, Debug)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

impl <T> Default for FetchState<T> {
    fn default() -> Self {
        FetchState::NotFetching
    }
}

impl <T> FetchState<T> {
    pub fn success(&self) -> Option<&T> {
        match self {
            FetchState::Success(value) => Some(value),
            _ => None
        }
    }

    pub fn unwrap(self) -> T {
        if let FetchState::Success(value) = self {
            value
        } else {
            panic!("Could not unwrap value of FetchState");
        }
    }

    pub fn map<U, F: Fn(T)-> U>(self, f: F ) -> FetchState<U> {
        match self {
            FetchState::NotFetching => FetchState::NotFetching,
            FetchState::Fetching => FetchState::NotFetching,
            FetchState::Success(t) => FetchState::Success(f(t)),
            FetchState::Failed(e) => FetchState::Failed(e)
        }
    }

    pub fn alter<F: Fn(&mut T)>(&mut self, f: F) {
        match self {
            FetchState::Success(t) => f(t),
            _ => {}
        }
    }

    pub fn as_ref(&self) -> FetchState<&T>  {
        match self {
            FetchState::NotFetching => FetchState::NotFetching,
            FetchState::Fetching => FetchState::NotFetching,
            FetchState::Success(t) => FetchState::Success(t),
            FetchState::Failed(e) => FetchState::Failed(e.clone())
        }
    }
}

impl <T: PartialEq> FetchState<T> {
    /// Sets the fetch state to be fetching.
    /// If it wasn't already in a fetch state, it will return `true`,
    /// to indicate that the component should re-render.
    pub fn set_fetching(&mut self) -> bool {
        self.neq_assign(FetchState::Fetching)
    }
}


// TODO add remaining HTTP methods.
pub enum MethodBody<'a, T> {
    Get,
    Delete,
    Post(&'a T),
    Put(&'a T)
}

impl <'a, T> MethodBody<'a, T> {
    pub fn as_method(&self) -> &'static str {
        match self {
            MethodBody::Get => "GET",
            MethodBody::Delete => "DELETE",
            MethodBody::Post(_) => "POST",
            MethodBody::Put(_) => "PUT"
        }
    }


}

impl <'a, T: Serialize> MethodBody<'a, T> {
    pub fn as_body(&self) -> Result<Option<JsValue>, FetchError> {
        let body: Option<String> = match self {
            MethodBody::Get
            | MethodBody::Delete => None,
            MethodBody::Put(data)
            | MethodBody::Post(data) => {
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
#[derive(Debug, PartialEq, Clone)]
pub enum FetchError {
    DeserializeError{error: String, content: String},
    TextNotAvailable,
    CouldNotCreateFetchFuture,
    CouldNotCreateRequest(JsValue),
    CouldNotSerializeRequestBody
}

pub trait FetchRequest {
    type RequestType: Serialize;
    type ResponseType: DeserializeOwned;

    fn url(&self) -> String;

    fn method(&self) -> MethodBody<Self::RequestType>;

    fn headers(&self) -> Vec<(String, String)>;

}

/// Fetch a resource, returning a result of the expected response,
/// or an error indicating what went wrong.
pub async fn fetch_resource<T: FetchRequest>(request: T) -> Result<T::ResponseType, FetchError> {
    log::debug!("fetch_resource");
    let method = request.method();
    let headers = request.headers();
    let headers = JsValue::from_serde(&headers).expect("Convert Headers to Tuple");

    // configure options for the request
    let mut opts = RequestInit::new();
    opts.method(method.as_method());
    opts.body(method.as_body()?.as_ref());
    opts.headers(&headers);

    opts.mode(RequestMode::Cors); // TODO make a thing for this, but its a fine default for the moment

    // Create the request
    let request = Request::new_with_str_and_init(
        &request.url(),
        &opts,
    )
        .map_err(|e| FetchError::CouldNotCreateRequest(e))?;


    // Send the request, resolving it to a response.
    let window: Window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| FetchError::CouldNotCreateFetchFuture)?;
    debug_assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // TODO, check the status code here.

    // Process the response
    let text = JsFuture::from(resp.text().map_err(|_| FetchError::TextNotAvailable)?)
        .await
        .map_err(|_| FetchError::TextNotAvailable)?;

    let text_string = text.as_string().unwrap();

    let deserialized = serde_json::from_str(&text_string)
        .map_err(|e| {
            FetchError::DeserializeError{error: e.to_string(), content: text_string}
        })?;

    Ok(deserialized)
}

/// Performs a fetch and then resolves the fetch to a message by way of using two provided Fns to
/// convert the success and failure cases.
///
/// This is useful if you want to handle the success case and failure case separately.
pub async fn fetch_to_msg<T: FetchRequest, Msg>(request: T, success: impl Fn(T::ResponseType) -> Msg, failure: impl Fn(FetchError) -> Msg) -> Msg {
    fetch_resource(request)
        .await
        .map(success)
        .unwrap_or_else(failure)
}

/// Performs a fetch and resolves the fetch to a message by converting a FetchState into the Message
/// by way of a provided closure.
///
/// This is useful if you just want to update a FetchState in your model based on the result of your request.
pub async fn fetch_to_state_msg<T: FetchRequest, Msg>(request: T, to_msg: impl Fn(FetchState<T::ResponseType>) -> Msg) -> Msg {
    let fetch_state = match fetch_resource(request).await {
        Ok(response) => FetchState::Success(response),
        Err(err) => FetchState::Failed(err)
    };

    to_msg(fetch_state)
}
