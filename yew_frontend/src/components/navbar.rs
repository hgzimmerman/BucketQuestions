use yew::prelude::*;
use wire::user::User;
use std::thread_local;
use yew_css::{Css, css_file};
use yewtil::NeqAssign;

thread_local! {
    static CSS: Css = css_file!("../../assets/navbar.css"); // TODO, not sure where the assets folder should go.
}



pub struct Navbar {
    props: Props,
    user: Option<User>
}

pub enum Msg {

}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub title: String
}



impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        Navbar {
            props,
            user: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html<Self> {
        CSS.with(|css| -> Html<Self> {
            html! {
                <nav class=&css["navbar"]>
                    <h3>{&self.props.title} </h3>
                </nav>
            }
        })
    }

}