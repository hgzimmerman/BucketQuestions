use yew::services::storage::{StorageService, Area};
use yew::services::fetch::{FetchService, Method, FetchTask, Request, Response};
use serde::{Serialize, Deserialize};
use yew::{Component, Callback, ComponentLink};
use yew::format::{Nothing, Json};

pub struct AuthenticationFetchService {
    local_storage: StorageService,
    fetch_service: FetchService
}

impl AuthenticationFetchService {
    pub fn new() -> Self {
        AuthenticationFetchService {
            local_storage: StorageService::new(Area::Local),
            fetch_service: FetchService::new()
        }
    }

    // TODO, change callback type to take a closure, also take a component link.
    // Implement login-redirect, and re-fetch of auth credentials using the link to create the actual callback.
    // Wrap the provided closure in another closure that has code for default handling of unauthorized requests.
    pub fn fetch<REQ, COMP, F>(&mut self, request: REQ, f: F, link: &mut ComponentLink<COMP>) -> FetchTask
    where
    REQ: FetchRequest,
    COMP: Component, // TODO, when does it start to make sense making this an actor...
    F: Fn(Result<REQ::ResponseBody, failure::Error>) -> COMP::Message + 'static
    {
        let requires_auth: bool = request.requires_auth();
        let path = request.path();

       let mut builder= Request::builder();
        builder
           .uri(path);

        if requires_auth {
            // TODO, if auth is required, check if the auth should be updated and do that first.
            let jwt_string: String = unimplemented!();
            builder.header("AUTHORIZATION", &jwt_string);
        };

        let callback = link.send_back(move |r: Response<Json<Result<REQ::ResponseBody, failure::Error>>>|{
            // TODO clean this boi up.
            if r.status().is_success() {
                let result = if let Json(body) = r.into_body() {
                    body
                } else {
                    panic!() //Err(Serialization fail)
                };
                (f)(result)
            } else {
                // TODO, if outdated auth, send redirect message.

                // TODO, add an error message indicating server failure with appropriate error code and message if applicable.
                panic!("not successs")
            }
        });

        match request.method() {
            HttpMethod::Get => {
                builder.method("GET");
                let request = builder.body(Nothing).unwrap();
                self.fetch_service.fetch(request, callback)
            },
            HttpMethod::Delete => {
                builder.method("DELETE");
                let request = builder.body(Nothing).unwrap();
                self.fetch_service.fetch(request, callback)
            },
            HttpMethod::Put(t) => {
                builder.method("PUT");
                let request = builder.body(Json(&t)).unwrap();
                self.fetch_service.fetch(request, callback)
            },
            HttpMethod::Post(t) => {
                builder.method("POST");
                let request = builder.body(Json(&t)).unwrap();
                self.fetch_service.fetch(request, callback)
            }
        }
    }
}

/// This is trait should probably be implemented on newtypes around the request body (if it is a Post/Put)
pub trait FetchRequest: Sized {
    type Body: Serialize;
    type ResponseBody: for<'de> Deserialize<'de> + 'static;
    fn method(self) -> HttpMethod<Self::Body>;
    fn requires_auth(&self) -> bool;
    fn path(&self) -> String;
}


/// Incomplete enumeration of http methods.
pub enum HttpMethod<T: Serialize> {
    Get,
    Delete,
    Put(T),
    Post(T)
}
