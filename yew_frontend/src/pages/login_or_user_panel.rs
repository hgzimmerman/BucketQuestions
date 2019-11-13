use crate::common::FetchState;
use yew::{Component, ComponentLink, Html, html, Properties, ShouldRender, Callback};
use yew::virtual_dom::VNode;
use wire::user::User;
use yewtil::ptr::Mrc;
use yewtil::NeqAssign;
use crate::pages::login::LoginPage;
use crate::pages::user_panel::UserPanel;

pub struct LoginUserPanel {
    props: Props
}

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub user: FetchState<User>,
    #[props(required)]
    pub callback: Callback<()>
}

pub enum Msg {

}

impl Component for LoginUserPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {

        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }


    fn view(&self) -> VNode<Self> {
        match self.props.user.clone().success() {
            Some(user) => html!{
                <UserPanel user = user callback = &self.props.callback />
            },
            None => html! {
                <LoginPage />
            }
        }
    }
}
