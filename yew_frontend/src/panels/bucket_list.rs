use yew::{Component, ComponentLink, Properties, Html, html};
use yew::virtual_dom::VNode;

pub struct BucketList {
    props: Props
}

#[derive(Debug, Properties)]
pub struct Props {

}

pub enum Msg {

}

impl Component for BucketList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }
    fn mounted(&mut self) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html<Self> {
        unimplemented!()
    }
}