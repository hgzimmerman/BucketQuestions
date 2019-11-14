use crate::common::FetchState;
use yew::{Component, ComponentLink, Html, html, Properties, ShouldRender, Callback};
use wire::user::User;
use yewtil::NeqAssign;
use crate::components::login::login::LoginPage;
use crate::components::login::user_panel::UserPanel;

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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
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


    fn view(&self) -> Html<Self> {
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
