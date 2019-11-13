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
use crate::common::{FetchState, fetch_resource, FetchError};

use wire::user::User;
use crate::requests::GetUser;
use crate::Msg::GotUserFailed;
use yewtil::NeqAssign;


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
    GotUserFailed(FetchError)
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
        let fetch = async {
            log::info!("Getting user");
            fetch_resource(&GetUser)
                .await
                .map(Msg::GotUser)
                .unwrap_or_else(Msg::GotUserFailed)
        };
        self.link.send_future(fetch);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotUser(user) => {
                log::info!("Got user");
                self.user.neq_assign(FetchState::Success(user))
            },
            Msg::GotUserFailed(err) => {
                log::warn!("Could not get user: {:?}", err);
                self.user.neq_assign(FetchState::Failed(err))
            }
        }
    }

    fn view(&self) -> Html<Self> {
//        let user = self.render_user();

        html!{
        <>
            <Navbar>
                <div style="flex-grow: 1">
                    <RouterLink
                        link = Route::from(AppRoute::Index).route
                        text = "BucketQuestions"
                    />
                </div>
                <div>
                    <RouterButton
                        link = Route::from(AppRoute::Login).route
                        text = "Login"
                    />
                </div>
            </Navbar>

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


impl Model {
    fn render_user<T: Component>(&self) -> Html<T> {
         self.user.get_success().map( |user| {
            return html! {
                {user.uuid}
            }
        })
             .unwrap_or_default()
    }
}
