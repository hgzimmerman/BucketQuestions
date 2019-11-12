use yew::{Component, ComponentLink, html, Html};
use yew::virtual_dom::VNode;
use crate::components::full_height::{full_height_scrollable};
use crate::components::centered::centered;
use crate::components::button::Button;
use yew_css::{Css, css_file};


// TODO the login page will likely be removed and replaced with a single button present in the navbar.

thread_local! {
    static CSS: Css = css_file!("../../assets/login_page.css");
}

// Get the oauth link from the server.
pub struct LoginPage {
    google_oauth_link: Option<String> // TODO probably the wrong data type.
}

pub enum Msg {
    GoToGoogleOauthPage
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LoginPage {
            google_oauth_link: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GoToGoogleOauthPage => {
                // go to page
                false
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
        full_height_scrollable(centered(html! {
            <div class = &css["card"]>
                <label>{"Google Login"}</label>
                <Button
                    callback = |_| Msg::GoToGoogleOauthPage
                    text= "Google Oauth"
                />
            </div>
        }))
    }
}
