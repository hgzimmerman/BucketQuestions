use yew::{Component, ComponentLink, html, ShouldRender};
use yew::virtual_dom::VNode;
use crate::common::{FetchState, fetch_to_msg, FetchError, fetch_to_state_msg};
use wire::bucket::Bucket;
use crate::requests::bucket::{GetPublicBuckets, GetParticipatingBuckets, CreateBucket};
use yewtil::NeqAssign;

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
