use yew::{Component, ComponentLink, html, ShouldRender, Html};
use yew::virtual_dom::VNode;
use yewtil::fetch::{FetchState, fetch_to_state_msg};
use wire::bucket::Bucket;
use crate::requests::bucket::{GetPublicBuckets, GetParticipatingBuckets};
use yewtil::NeqAssign;
use crate::AppRoute;
use yew_router::agent::RouteRequest;
use crate::auth::is_logged_in;

pub struct IndexPage {
    public_buckets: FetchState<Vec<Bucket>>,
    users_buckets: FetchState<Vec<Bucket>>,
    /// For holding failure values for the create bucket request
//    create_bucket: FetchState<()>,
    link: ComponentLink<Self>
}

#[derive(Debug, Clone)]
pub enum Msg {
    FetchedPublicBuckets(FetchState<Vec<Bucket>>),
    FetchedUserBuckets(FetchState<Vec<Bucket>>),
//    RequestCreateBucket(CreateBucket),
//    FetchedCreatedBucket(FetchState<Bucket>),
    GoTo(AppRoute)
}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        IndexPage {
            public_buckets: Default::default(),
            users_buckets: Default::default(),
//            create_bucket: Default::default(),
            link,
        }
    }


    fn mounted(&mut self) -> bool {
        self.public_buckets.set_fetching();
        let fetch = fetch_to_state_msg(GetPublicBuckets, Msg::FetchedPublicBuckets);
        self.link.send_future(fetch);

        self.users_buckets.set_fetching();
        let fetch = fetch_to_state_msg(GetParticipatingBuckets, Msg::FetchedUserBuckets);
        self.link.send_future(fetch);

        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchedPublicBuckets(state) => self.public_buckets.neq_assign(state),
            Msg::FetchedUserBuckets(state) => self.users_buckets.neq_assign(state),
//            Msg::RequestCreateBucket(create_bucket) => {
//                self.create_bucket.set_fetching();
//                let fetch = fetch_to_state_msg(create_bucket, Msg::FetchedCreatedBucket);
//                self.link.send_future(fetch);
//                false
//            }
//            Msg::FetchedCreatedBucket(bucket) => {
//                match bucket {
//                    FetchState::Success(bucket) => {
//                        // Add either to the bucket.
//                        self.users_buckets.alter(|buckets| {
//                            buckets.push(bucket.clone());
//                        });
//                        self.public_buckets.alter(|buckets| {
//                            buckets.push(bucket.clone());
//                        });
//                        true
//                    }
//                    FetchState::Failed(err) => self.create_bucket.neq_assign(FetchState::Failed(err)),
//                    _ => unreachable!()
//                }
//            }
            Msg::GoTo(route) => {
                yew_router::unit_state::RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute(route.into()));
                false
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        let public_buckets = match &self.public_buckets {
            FetchState::Success(buckets) => {
                buckets.iter().map(Self::bucket_card).collect::<Html<Self>>()
            }
            FetchState::NotFetching => {
                html!{}
            }
            FetchState::Fetching => {
                html!{}
            }
            FetchState::Failed(e) => {
                html!{format!("{:?}",e)}
            }
        };

        let users_buckets = match &self.users_buckets {
            FetchState::Success(buckets) => {
                buckets.iter().map(Self::bucket_card).collect::<Html<Self>>()
            }
            FetchState::NotFetching => {
                html!{}
            }
            FetchState::Fetching => {
                html!{}
            }
            FetchState::Failed(e) => {
                html!{format!("{:?}",e)}
            }
        };

        html! {
            <div class= "full_height has-background-primary">
                <div class = "container full_height2" style="width: 100%; padding-top: 10px;">
                    <div class = "columns full_height3 is-marginless">
                        <div class = "column full_height2">
                            <div class="card full_height2 vert_flex">
                                <div class="card-header">
                                    <p class="card-header-title">{"Public Buckets"}</p>
                                </div>

                                <div class="card-content full_height2 is-paddingless">
                                    <div class="panel full_height_scrollable2">
                                        {public_buckets}
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class = "column full_height2">
                            <div class = "card full_height2 vert_flex">
                                <div class="card-header">
                                    <p class="card-header-title">{"Private Buckets"}</p>
                                </div>

                                <div class="card-content full_height2 is-paddingless">
                                    <div class="panel full_height_scrollable2">
                                        {users_buckets}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                {Self::create_bucket_button()}
            </div>
        }
    }
}


impl IndexPage {
    fn bucket_card(bucket: &Bucket) -> Html<Self> {
        let slug = bucket.bucket_slug.clone();
        let route = AppRoute::Bucket{slug};
        html! {
            <a
                class = "panel-block is-white"
                onclick = |_| Msg::GoTo(route.clone())
            >
                <label class="is-size-5">{&bucket.bucket_name} </label>
            </a>
        }
    }

    fn create_bucket_button() -> Html<Self> {
        if is_logged_in() {
            // TODO add a tooltip.
            return html! {
                <a
                    class = "static_corner"
                    onclick= |_| Msg::GoTo(AppRoute::CreateBucket)
                >
                    <div class= "has-background-info circle_button">
                        <span class="icon has-text-info full_size" >
                            <i class="fas fa-plus big_icon" style = "color: white"></i>
                        </span>
                    </div>
                </a>
            }
        } else {
            return html!{}
        }
    }
}