use yew::{Component, ComponentLink, html, ShouldRender};
use yew::virtual_dom::VNode;

pub struct IndexPage {

}

pub enum Msg {

}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        IndexPage {

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

        }
    }

    fn view(&self) -> VNode<Self> {
        html! {
            <div>
                <h1> {"Welcome to Bucket Questions!"} </h1>
                <h2> {"The only site to have both buckets, and questions."} </h2>
                <h2> {"Some sites have buckets, others have questions, this one has both."} </h2>

                <div>
                    <p> {"Your buckets"}</p>
                    <p> {"No bucket, create one!"} </p>

                </div>

                <div>
                    <label> {"Public buckets"}</label>
                </div>
            </div>
        }
    }
}
