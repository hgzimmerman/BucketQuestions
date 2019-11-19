use yew::{Component, ComponentLink, Properties, html};
use yew::virtual_dom::VNode;
use yewtil::NeqAssign;
use yewtil::fetch::{FetchState, fetch_to_state_msg};
use wire::bucket::{Bucket, NewBucketRequest};
use crate::requests::bucket::CreateBucket;
use crate::pages::create_bucket::Msg::FetchedCreateBucket;
use yew_router::unit_state::RouteAgentDispatcher;
use yew_router::agent::RouteRequest;
use crate::AppRoute;

pub struct CreateBucketPage {
    bucket_name: String,
    create_bucket: FetchState<Bucket>,
    link: ComponentLink<CreateBucketPage>
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
}

pub enum Msg {
    UpdateBucketName(String),
    FetchCreateBucket,
    FetchedCreateBucket(FetchState<Bucket>)
}

impl Component for CreateBucketPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            bucket_name: "".to_string(),
            create_bucket: Default::default(),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateBucketName(bucket_name) => self.bucket_name.neq_assign(bucket_name),
            Msg::FetchCreateBucket => {
                self.create_bucket.set_fetching();
                let request = CreateBucket(NewBucketRequest {
                    bucket_name: self.bucket_name.clone()
                });
                let fetch = fetch_to_state_msg(request, FetchedCreateBucket);
                self.link.send_future(fetch);
                true
            }
            Msg::FetchedCreateBucket(state) => {
                self.create_bucket.neq_assign(state);
                if let FetchState::Success(bucket) = &self.create_bucket {
                    let route = AppRoute::Bucket { slug: bucket.bucket_slug.clone() };
                    RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute(route.into()));
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        html! {
            <div class= "has-background-primary full_height_scrollable">
                <div class = "columns is-centered full_width is-marginless">
                    <div class="column is-two-thirds-tablet is-half-desktop">
                        <div class = "card min_height_200">
                            <div class="card-header">
                                <p class="card-header-title">
                                    {"Create Bucket"}
                                </p>
                            </div>
                            <div class="card-content">
                                <div class="field">
                                    <label class="label">{"Bucket Name"}</label>
                                    <div class="control">
                                        <input class="input"
                                            type="text"
                                            value=self.bucket_name
                                            oninput = |i| Msg::UpdateBucketName(i.value)
                                        />
                                    </div>
                                    <div class="field is-grouped is-grouped-centered">
                                        <p class="control">
                                            <a class="button is-primary" onclick=|_| Msg::FetchCreateBucket>
                                                {"Submit"}
                                            </a>
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}