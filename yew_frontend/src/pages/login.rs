use yew::{Component, ComponentLink, html, Html, ShouldRender};
use yew::virtual_dom::VNode;
use crate::components::full_height::{full_height_scrollable};
use crate::components::centered::centered;
use crate::components::button::Button;
use yew_css::{Css, css_file};
use crate::common::{fetch_resource, FetchError, FetchState};
use crate::requests::LinkResponse;
use std::future::Future;
use yewtil::NeqAssign;
use web_sys::{Window, Location};


// TODO the login page will likely be removed and replaced with a single button present in the navbar.

thread_local! {
    static CSS: Css = css_file!("../../assets/login_page.css");
}


// Get the oauth link from the server.
pub struct LoginPage {
    google_oauth_link: FetchState<String>, // TODO probably the wrong data type.
    link: ComponentLink<LoginPage>
}

pub enum Msg {
    NoOp,
    GoToGoogleOauthPage,
    GotLink(LinkResponse),
    GotLinkFail(FetchError)
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LoginPage {
            google_oauth_link: FetchState::default(),
            link
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let future = async {
            let request = crate::requests::GetOauthLink;
            fetch_resource(&request)
                .await
                .map(Msg::GotLink)
                .unwrap_or_else(Msg::GotLinkFail)
        };
        self.link.send_future(future);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NoOp => {
                false
            }
            Msg::GoToGoogleOauthPage => {
                // go to page
                log::info!("Going to google's oauth page");
                let window: Window = web_sys::window().unwrap();
                window.location().assign(self.google_oauth_link.as_ref().unwrap());
                false
            }
            Msg::GotLink(link) => {
                self.google_oauth_link.neq_assign( FetchState::Success(link.link))
            }
            Msg::GotLinkFail(err) => {
                log::error!("{:?}", err);
                self.google_oauth_link.neq_assign(FetchState::Failed(err))
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        CSS.with(|css| -> Html<Self> {
           self.css_view(css)
        })
    }
}

impl LoginPage {
    fn css_view(&self, css: &Css) -> Html<Self> {
//        full_height_scrollable(centered(
            match &self.google_oauth_link {
                FetchState::NotFetching
                | FetchState::Fetching => html! {
                    <Button
                        callback = |_| Msg::NoOp
                        text= "Login"
                    />
                },
                FetchState::Success(_) => html! {
                    <Button
                        callback = |_| Msg::GoToGoogleOauthPage
                        text= "Login"
                    />
                },
                FetchState::Failed(_err) => html!{}
            }
//        ))
    }
}
