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

mod requests;

use crate::pages::login::LoginPage;
use crate::pages::index::IndexPage;

use yew_router::prelude::{RouterButton, Route};

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


pub struct Model {}

pub enum Msg {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
        }
    }

    fn view(&self) -> Html<Self> {
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
