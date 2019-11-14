use yew::prelude::*;
use std::thread_local;
use yew_css::{Css, css_file};
//use yewtil::NeqAssign;
//use crate::components::button::Button;


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

pub fn navbar<T: Component>(html: Html<T>) -> Html<T> {
    CSS.with(|css| -> Html<T> {
        html! {
            <nav class=&css["navbar"]>
                {html}
            </nav>
        }
    })
}

