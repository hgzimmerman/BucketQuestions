use yew::prelude::*;
use std::thread_local;
use yew_css::{Css, css_file};
//use yewtil::NeqAssign;
//use crate::components::button::Button;
use crate::{Model, Msg};
use crate::components::login::login_or_user_panel::LoginUserPanel;
use yew_router::unit_state::{RouterLink, Route};
use crate::AppRoute;

thread_local! {
    static CSS: Css = css_file!("../../assets/navbar.css"); // TODO, not sure where the assets folder should go.
}


impl Model {
    pub fn navbar(&self) -> Html<Model> {
        html! {
            <nav class="navbar is-dark" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                        // TODO make this a bread crumbs thing.
                        <RouterLink
                            link = Route::from(AppRoute::Index).route
                            text = "Bucket Questions"
                            classes = "is-primary navbar-item"
                        />
                    {self.render_burger()}
                </div>
                {self.render_nav_or_burger_content()}
            </nav>
        }
    }

    fn render_burger(&self) -> Html<Self> {

        let inner_content = html! {
            <>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
            </>
        };
        if self.burger_open {
            return html! {
                <a role="button" class="navbar-burger burger is-active" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample" onclick = |_| Msg::ToggleBurger>
                   {inner_content}
                </a>
            }
        } else {
            return html! {
                <a role="button" class="navbar-burger burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample" onclick = |_| Msg::ToggleBurger>
                   {inner_content}
                </a>
            }
        }
    }

    fn render_nav_or_burger_content(&self) -> Html<Self> {

        // TODO make this conditional so it only shows up in index
        let create_bucket = html! {
//            <RouterLink
//                link = Route::from(AppRoute::CreateBucket).route
//                text = "Create Bucket"
//                classes = "navbar-item"
//            />
        };
        let inner_content = html! {
        <>
            <div class="navbar-start">
            // TODO, maybe put the name of the bucket here? breadcrumbs?
                {create_bucket}
            </div>

            <div class="navbar-end">
                <LoginUserPanel user = &self.user callback=|_| Msg::LogUserOut />
            </div>
        </>
        };

        if self.burger_open {
            return html! {
                <div id="navbarBasicExample" class="navbar-menu is-active">
                   {inner_content}
                </div>
            }
        } else {
            return html! {
                <div id="navbarBasicExample" class="navbar-menu">
                    {inner_content}
                </div>
            }
        }
    }
}

