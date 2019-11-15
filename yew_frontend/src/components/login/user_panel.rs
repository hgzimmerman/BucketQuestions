use yew::{Component, ComponentLink, Html, html, Properties, Callback, Classes};
use yew::virtual_dom::VNode;
use wire::user::User;
use crate::components::button::Button;


pub struct UserPanel {
    props: Props,
    open: bool
}

#[derive(Debug, Properties)]
pub struct Props {
    #[props(required)]
    pub user: User,
    #[props(required)]
    pub callback: Callback<()>
}

pub enum Msg {
    ToggleOpen,
    LogOut
}

impl Component for UserPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            open: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleOpen => {self.open = !self.open; true}
            Msg::LogOut => {
                self.props.callback.emit(());
                true
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        html! {
            <>
            {
                self.render_user()
            }
            {
                self.render_panel()
            }
            </>
        }
    }
}

impl UserPanel {
    fn render_user(&self) -> Html<Self> {

        let user_name: &String = &self.props.user.google_name.clone().unwrap_or_else(|| "Logged In".to_string());

        return html! {
            <a
                class = Classes::from("navbar-item")
                onclick = |_| Msg::ToggleOpen
            >
                {user_name}
            </a>
        }
    }

    fn render_panel(&self) -> Html<Self> {
        if self.open {
            return html! {
                <div style = "position: absolute; top: 44px; background-color: orange; height: 120px; width: 100%;">
                    <Button
                        callback = |_| Msg::LogOut
                        text = "Log Out"
                    />
                </div>
            }
        } else {
            return html!{}
        }

    }
}
