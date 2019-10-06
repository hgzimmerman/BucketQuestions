use yew::prelude::*;

mod components;
mod services;
mod agents;
use components::navbar::Navbar;

pub fn start_app() {
    web_logger::init();
    yew::start_app::<Model>();
}

pub struct Model {}

pub enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html!{
            <Navbar title = "Bucket Questions" />
        }
    }
}
