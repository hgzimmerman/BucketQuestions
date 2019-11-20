use yew::{Component, ComponentLink, Properties, html, Html};
use yew::virtual_dom::VNode;
use yewtil::NeqAssign;
use wire::question::{Question, NewQuestionRequest};
use yewtil::fetch::{FetchState, fetch_to_state_msg};
use crate::requests::question::{GetRandomQuestion, CreateQuestion, DeleteQuestion, GetNumberOfQeustionsInTheBucket};
use wire::bucket::Bucket;
use crate::requests::bucket::{GetBucketBySlug, GetPermissionsForUser, AddSelfToBucket};
use uuid::Uuid;
use crate::requests::answer::CreateAnswer;
use wire::answer::NewAnswerRequest;
use crate::pages::settings_modal::SettingsModal;
use yew_router::unit_state::{RouteAgentDispatcher, Route};
use crate::AppRoute;
use yew_router::agent::RouteRequest;
use wire::bucket_user_relation::BucketUserPermissions;
use crate::pages::bucket::control::join::{JoinAction, JoinLogic};
use crate::pages::bucket::control::answer::{AnswerAction, AnswerState};
use crate::pages::bucket::control::new_question::{NewQuestionAction, NewQuestionState};
use crate::pages::bucket::control::num_questions::{NumQuestionAction, NumQuestionsState};
use crate::pages::bucket::control::permissions::{PermissionsAction, PermissionsState};
use crate::pages::bucket::control::active_question::{ActiveQuestionState, ActiveQuestionAction};


mod view;
mod control;

// TODO break this component across a data-wiring, view and update modules. @ 500 lines, this is getting hard to follow.
pub struct BucketPage {
    props: Props,
    link: ComponentLink<BucketPage>,
    bucket: FetchState<Bucket>,
    new_answer: AnswerState,
    new_question: NewQuestionState,
    num_questions: NumQuestionsState,
    permissions: PermissionsState,
    active_question: ActiveQuestionState
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub is_settings_open: bool
}

pub enum Msg {
    FetchedBucket(FetchState<Bucket>),
    ShowSettingsModal,
    Joining(JoinAction),
    Answer(AnswerAction),
    NewQuestion(NewQuestionAction),
    NumQuestions(NumQuestionAction),
    Permissions(PermissionsAction),
    ActiveQuestion(ActiveQuestionAction)
}

impl Component for BucketPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            bucket: Default::default(),
            active_question: Default::default(),
            permissions: Default::default(),
            new_answer: Default::default(),
            new_question: Default::default(),
            num_questions: Default::default()
        }
    }

    fn mounted(&mut self) -> bool {
        let request = GetBucketBySlug{slug: self.props.slug.clone()};
        let fetch = fetch_to_state_msg(request, Msg::FetchedBucket);
        self.link.send_future(fetch);
        false
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        let bucket_ref = self.bucket.as_ref();
        let get_bucket_uuid = || bucket_ref.success().map(|bucket| bucket.uuid);
        match msg {
            Msg::FetchedBucket(state) => self.handle_fetched_bucket(state),
            Msg::ShowSettingsModal => self.show_settings_modal(),
            Msg::Joining(action) => JoinLogic::update(action, &mut self.link, &self.bucket),
            Msg::Answer(action) => self.new_answer.update(action, &mut self.link, &mut self.active_question.0),
            Msg::NewQuestion(action) => self.new_question.update(action, &mut self.link, &self.bucket),
            Msg::NumQuestions(action) => self.num_questions.update(action, &mut self.link, get_bucket_uuid()),
            Msg::Permissions(action) => self.permissions.update(action, &mut self.link, get_bucket_uuid()),
            Msg::ActiveQuestion(action) => self.active_question.update(action, &mut self.link, get_bucket_uuid())
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html<Self> {
        self.view_impl()
    }
}

