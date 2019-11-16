use yew::{Component, ComponentLink, html, ShouldRender, Html};
use yew::virtual_dom::VNode;
use crate::common::{FetchState, fetch_to_msg, FetchError, fetch_to_state_msg};
use wire::bucket::Bucket;
use crate::requests::bucket::{GetPublicBuckets, GetParticipatingBuckets, CreateBucket};
use yewtil::NeqAssign;
use yew_router::unit_state::{Route, RouterLink};
use crate::AppRoute;

pub struct IndexPage {
    public_buckets: FetchState<Vec<Bucket>>,
    users_buckets: FetchState<Vec<Bucket>>,
    /// For holding failure values for the create bucket request
    create_bucket: FetchState<()>,
    link: ComponentLink<Self>
}

pub enum Msg {
    FetchedPublicBuckets(FetchState<Vec<Bucket>>),
    FetchedUserBuckets(FetchState<Vec<Bucket>>),
    RequestCreateBucket(CreateBucket),
    FetchedCreatedBucket(FetchState<Bucket>)
}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        IndexPage {
            public_buckets: Default::default(),
            users_buckets: Default::default(),
            create_bucket: Default::default(),
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
            Msg::RequestCreateBucket(create_bucket) => {
                self.create_bucket.set_fetching();
                let fetch = fetch_to_state_msg(create_bucket, Msg::FetchedCreatedBucket);
                self.link.send_future(fetch);
                false
            }
            Msg::FetchedCreatedBucket(bucket) => {
                match bucket {
                    FetchState::Success(bucket) => {
                        // Add either to the bucket.
                        self.users_buckets.alter(|buckets| {
                            buckets.push(bucket.clone());
                        });
                        self.public_buckets.alter(|buckets| {
                            buckets.push(bucket.clone());
                        });
                        true
                    }
                    FetchState::Failed(err) => self.create_bucket.neq_assign(FetchState::Failed(err)),
                    _ => unreachable!()
                }
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
            <div class= "full_height">
                <div class = "container" style="width: 100%; padding-top: 10px;">
                    <div class = "columns full_height_scrollable2">
                        <div class = "column full_height2">
                            <div class="panel is-primary full_height2 vert_flex">
                                <p class="panel-heading">{"Public Buckets"}</p>
                                <div class = "growable_scrollable">
                                    {public_buckets}
                                </div>
                            </div>
                        </div>

                        <div class = "column full_height2">
                            <div class="panel is-primary full_height2 vert_flex">
                                <p class="panel-heading">{"Private Buckets"}</p>
                                <div class = "growable_scrollable">
                                    {users_buckets}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}


impl IndexPage {
    fn bucket_card(bucket: &Bucket) -> Html<Self> {
        html! {
            <div class = "panel-block is-white">
                <label class="is-size-7">{&bucket.bucket_name} </label>
                <RouterLink
                    link = Route::from(AppRoute::Bucket{slug: bucket.bucket_slug.clone()}).route
                    text = "Go To"
                />
            </div>
        }

    }
}