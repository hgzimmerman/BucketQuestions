use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;

mod common;

mod components;
mod services;
mod agents;
use components::navbar::Navbar;
mod pages;

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
                <RouterLink
                    link = Route::from(AppRoute::Index).route
                    text = "BucketQuestions"
                />
                <RouterButton
                    link = Route::from(AppRoute::Login).route
                    text = "Login"
                />
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
