#![recursion_limit="256"]
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;

mod common;

mod components;
mod services;
mod agents;
use components::navbar::Navbar;
mod pages;
mod auth;

mod requests;

use crate::pages::login::LoginPage;
use crate::pages::index::IndexPage;

use yew_router::prelude::{RouterButton, Route};
use crate::common::{FetchState, fetch_resource, FetchError, fetch_to_msg};

use wire::user::User;
use crate::requests::GetUser;
use crate::Msg::GotUserFailed;
use yewtil::NeqAssign;
use yewtil::ptr::Mrc;
use crate::pages::login_or_user_panel::LoginUserPanel;
use crate::components::navbar::navbar;
use crate::common::FetchState::Fetching;


#[wasm_bindgen]
pub fn start_app() {
    web_logger::init();
    yew::start_app::<Model>();
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/"]
    Index,
}


pub struct Model {
    user: FetchState<User>,
    link: ComponentLink<Self>
}

pub enum Msg {
    GotUser(User),
    GotUserFailed(FetchError),
    LogUserOut
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            user: Default::default(),
            link
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let fetch = fetch_to_msg(&GetUser, Msg::GotUser, Msg::GotUserFailed);
        self.link.send_future(fetch);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotUser(user) => {
                log::info!("Got user: {:#?}", user);
                self.user.neq_assign(FetchState::Success(user))
            },
            Msg::GotUserFailed(err) => {
                log::warn!("Could not get user: {:?}", err);
                self.user.neq_assign(FetchState::Failed(err))
            }
            Msg::LogUserOut => {
                crate::auth::clear_jwt();
                self.user.neq_assign(FetchState::NotFetching)
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html!{
        <>
            {navbar(html!{<>
                <div style="flex-grow: 1">
                    <RouterLink
                        link = Route::from(AppRoute::Index).route
                        text = "BucketQuestions"
                    />
                </div>
                <div>
                    <LoginUserPanel user = &self.user callback=|_| Msg::LogUserOut />
                </div>
            </>})}

            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Login => html!{<LoginPage/>},
                        AppRoute::Index => html!{<IndexPage/>},
                    }
                })
                redirect = Router::redirect(|_| {
                    AppRoute::Index
                })
            />
        </>
        }
    }
}

