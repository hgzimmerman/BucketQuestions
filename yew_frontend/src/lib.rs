#![recursion_limit="512"]
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;

mod common;

mod components;
mod pages;
mod auth;

mod requests;
mod panels;

use crate::pages::index::IndexPage;

use yew_router::prelude::{Route};
use crate::common::{FetchState,  FetchError, fetch_to_msg};

use wire::user::User;
use crate::requests::auth_and_user::GetUser;
use yewtil::NeqAssign;
use crate::components::login::login_or_user_panel::LoginUserPanel;
//use crate::components::navbar::navbar;
use crate::pages::bucket::BucketPage;
use crate::pages::create_bucket::CreateBucketPage;

#[wasm_bindgen]
pub fn start_app() {
    web_logger::init();
    yew::start_app::<Model>();
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/!"]
    Index,
    #[to = "/bucket/{slug}"]
    Bucket{slug: String},
    #[to = "/create_bucket"]
    CreateBucket
}


pub struct Model {
    user: FetchState<User>,
    burger_open: bool,
    link: ComponentLink<Self>
}

pub enum Msg {
    GotUser(User),
    GotUserFailed(FetchError),
    LogUserOut,
    ToggleBurger
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            user: Default::default(),
            burger_open: false,
            link
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let fetch = fetch_to_msg(GetUser, Msg::GotUser, Msg::GotUserFailed);
        self.link.send_future(fetch);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotUser(user) => {
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
            Msg::ToggleBurger => {
                self.burger_open  = !self.burger_open;
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html!{
        <>
            {self.navbar()}
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html!{<IndexPage/>},
                        AppRoute::Bucket{slug} => html!{<BucketPage slug = slug />},
                        AppRoute::CreateBucket => html!{<CreateBucketPage />}
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

