use yew::virtual_dom::VNode;
use yew::{html, Callback, Properties};
use yew::Classes;
use yewtil::{Emissive, Pure, PureComponent};

pub type Button = Pure<PureButton>;

#[derive(PartialEq, Properties, Emissive)]
pub struct PureButton {
    #[props(required)]
    pub callback: Callback<()>,
    pub text: String,
//    pub classes: Classes
}

impl PureComponent for PureButton {
    fn render(&self) -> VNode<Pure<Self>> {
        html! {
            <button
//                classes = self.classes
                onclick=|_| ()>
                { &self.text }
            </button>
        }
    }
}