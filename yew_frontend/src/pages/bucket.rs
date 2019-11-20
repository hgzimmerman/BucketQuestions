use yew::{Component, ComponentLink, Properties, html, Html};
use yew::virtual_dom::VNode;
use yewtil::NeqAssign;
use wire::question::{Question, NewQuestionRequest};
use yewtil::fetch::{FetchState, fetch_to_state_msg};
use crate::requests::question::{GetRandomQuestion, CreateQuestion, DeleteQuestion, GetNumberOfQeustionsInTheBucket};
use crate::pages::bucket::Msg::{FetchedActiveQuestion, GetPermissions};
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
use crate::pages::bucket::control::join::{JoinAction, Join};
use crate::pages::bucket::control::answer::{AnswerAction, Answer};


mod view;
mod control;

// TODO break this component across a data-wiring, view and update modules. @ 500 lines, this is getting hard to follow.
pub struct BucketPage {
    props: Props,
    link: ComponentLink<BucketPage>,
    bucket: FetchState<Bucket>,
    active_question: FetchState<Option<Question>>,
    new_question: String,
    new_question_create: FetchState<()>,
    new_answer: Answer,
    questions_in_bucket_count: FetchState<usize>,
    permissions: FetchState<BucketUserPermissions>
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
    FetchedActiveQuestion(FetchState<Option<Question>>),
    UpdateNewQuestion(String),
    FetchedNewQuestionCreate(FetchState<()>),
    GetARandomQuestion,
    PutQuestionBackInBucket,
    GetPermissions,
    FetchedPermissions(FetchState<BucketUserPermissions>),
    GetNumQuestionsInBucket,
    FetchedNumQuestionsInBucket(FetchState<usize>),
    DiscardQuestion,
    FetchedDiscardQuestion(FetchState<Question>),
    SubmitNewQuestion,
    ShowSettingsModal,
    Joining(JoinAction),
    Answer(AnswerAction)
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
            new_question: "".to_string(),
            new_question_create: Default::default(),
            questions_in_bucket_count: Default::default(),
            permissions: Default::default(),
            new_answer: Default::default()
        }
    }

    fn mounted(&mut self) -> bool {
        let request = GetBucketBySlug{slug: self.props.slug.clone()};
        let fetch = fetch_to_state_msg(request, Msg::FetchedBucket);
        self.link.send_future(fetch);
        false
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchedBucket(state) => self.handle_fetched_bucket(state),
            Msg::FetchedActiveQuestion(state) => self.handle_fetched_active_question(state),
            Msg::UpdateNewQuestion(question_text) => self.new_question.neq_assign(question_text),
            Msg::FetchedNewQuestionCreate(new_question) => self.handle_post_new_question(new_question),
            Msg::GetARandomQuestion => self.get_a_random_question(),
            Msg::PutQuestionBackInBucket => self.put_question_in_bucket(),
            Msg::DiscardQuestion => self.discard_question(),
            Msg::SubmitNewQuestion => self.submit_question(),
            Msg::ShowSettingsModal => self.show_settings_modal(),
            Msg::FetchedPermissions(permissions) => self.permissions.neq_assign(permissions),
            Msg::GetPermissions => self.get_user_permissions(),
            Msg::GetNumQuestionsInBucket => self.get_num_questions_in_bucket(),
            Msg::FetchedNumQuestionsInBucket(num) => self.questions_in_bucket_count.neq_assign(num),
            Msg::FetchedDiscardQuestion(question) => self.fetched_discarded_question(question),
            Msg::Joining(action) => Join::update(action, &mut self.link, &self.bucket),
            Msg::Answer(action) => self.new_answer.update(action, &mut self.link, &mut self.active_question)
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html<Self> {
        self.view_impl()
    }
}

