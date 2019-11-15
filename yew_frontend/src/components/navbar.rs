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



//pub struct Navbar {
//    props: Props,
//}
//
//pub enum Msg {
//}
//
//#[derive(Debug, Properties)]
//pub struct Props {
//    pub children: Children<Navbar>,
//}
//
//
//
//impl Component for Navbar {
//    type Message = Msg;
//    type Properties = Props;
//
//    fn create(props: Props, link: ComponentLink<Self>) -> Self {
//        Navbar {
//            props,
//        }
//    }
//
//    fn update(&mut self, msg: Self::Message) -> ShouldRender {
//        true
//    }
//
//    fn change(&mut self, props: Self::Properties) -> ShouldRender {
//        self.props = props;
//        true
////        self.props.neq_assign(props)
//    }
//
//    fn view(&self) -> Html<Self> {
//        CSS.with(|css| -> Html<Self> {
//            return html! {
//                <nav class=&css["navbar"]>
//                    {self.props.children.render()}
//                </nav>
//            }
//        })
//    }
//}

impl Model {
    pub fn navbar(&self) -> Html<Model> {
        html! {
            <nav class="navbar is-dark" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
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
        let inner_content = html! {
        <>
            <div class="navbar-start">
                <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">
                        {"More"}
                    </a>

                    <div class="navbar-dropdown">
                        <a class="navbar-item">
                            {"About"}
                        </a>
                        <a class="navbar-item">
                            {"Jobs"}
                        </a>
                        <a class="navbar-item">
                            {"Contact"}
                        </a>
                        <hr class="navbar-divider" />

                        <a class="navbar-item">
                            {"Report an issue"}
                        </a>
                    </div>
                </div>
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

