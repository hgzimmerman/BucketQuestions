use yew::{Component, ComponentLink, Properties, html, Html, ShouldRender};
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
use crate::pages::bucket::join_logic::{JoinAction, JoinLogic};
use crate::pages::bucket::answer::{AnswerAction, AnswerState};
use crate::pages::bucket::new_question::{NewQuestionAction, NewQuestionState};
use crate::pages::bucket::num_questions::{NumQuestionAction, NumQuestionsState};
use crate::pages::bucket::permissions::{PermissionsAction, PermissionsState, SettingsJoin};
use crate::pages::bucket::active_question::{ActiveQuestionState, ActiveQuestionAction};


mod new_question;
mod active_question;
mod permissions;
mod num_questions;
mod join_logic;
mod answer;

/// Shorthand alias for the link argument.
type BucketLink = ComponentLink<BucketPage>;

/// Model for displaying bucket related data.
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


impl BucketPage {
    pub fn handle_fetched_bucket(&mut self, state: FetchState<Bucket>) -> ShouldRender {
        let rerender = self.bucket.neq_assign(state);

        // get permissions and number of questions.
//                self.link.send_back_batch(|_| vec![Msg::GetPermissions, Msg::GetNumQuestionsInBucket]).emit(());
        // TODO, we need send_batch
        self.link.send_self(Msg::Permissions(PermissionsAction::Get));
        self.link.send_self(Msg::NumQuestions(NumQuestionAction::Get));

        rerender
    }

    pub fn show_settings_modal(&mut self) -> ShouldRender {
        let route = AppRoute::BucketSettings{ slug: self.props.slug.clone() };
        RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute( Route::from(route)));
        false
    }
}

impl BucketPage {

    pub fn view_impl(&self) -> Html<Self> {
        html! {
            <>
                {self.modal()}
                <div class= "has-background-primary full_height_scrollable">
                    <div class = "full_width">
                        <div class = "columns is-centered no_margin">
                            <div class="column is-two-thirds-tablet is-half-desktop is-centered">
                                {self.render_title()}
                                {self.active_question.render_q_and_a_card(&self.new_answer)} // TODO, consider moving the new answer inside of the active_question struct.
                                {self.new_question.render_new_question_card()}
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn modal(&self) -> Html<Self> {
        if let (FetchState::Success(bucket), FetchState::Success(permissions) )= (&self.bucket, &self.permissions.permissions) {
            if self.props.is_settings_open {
                return html!{
                    <SettingsModal bucket= bucket.clone() permissions = permissions.clone()/>
                }
            } else {
                return html!{}
            }
        } else {
            return html!{}
        }
    }

    fn render_title(&self) -> Html<Self> {
        let settings_modal_link_or_join_button = match self.permissions.show_settings_or_join_button() {
            Some(SettingsJoin::Settings) => html! {
                <a
                    onclick=|_| Msg::ShowSettingsModal
                    href="#" class="card-header-icon" aria-label="bucket settings"
                >
                    <span class="icon has-text-dark">
                        <i class="fas fa-cog" aria-hidden="true"></i>
                    </span>
                </a>
            },
            Some(SettingsJoin::JoinBucket) => html!{
                <div class="full_height">
                    <button
                        class="button"
                        onclick=|_| Msg::Joining(JoinAction::Join)
                    >
                        {"Join"}
                    </button>
                </div>
            },
            None => html!{}
        } ;

        let num_questions_in_bucket = if let FetchState::Success(count) = self.num_questions.num_questions {
            html! {
                <span class= "" style = "padding-top: .75rem; padding-bottom: .75rem; padding-right: .25rem">
                    {format!("Q: {}", count)}
                </span>
            }
        } else {
            html!{}
        };

        let content = match &self.bucket {
            FetchState::Success(bucket) => html !{
                html!{
                    <>
                        <span class="card-header-title">
                            {&bucket.bucket_name}
                        </span>
                        {num_questions_in_bucket}
                        {settings_modal_link_or_join_button}
                    </>
                }
            },
            _ => html!{
                <>
                    <span class="card-header-title">
                        <progress class="progress is-small is-dark is-radiusless" max="100"></progress>
                        {crate::NBS}
                    </span>
                    {settings_modal_link_or_join_button}
                </>
            }
        };

        html! {

            <div class="card column_margin">
                <header class="card-header">
                    {content}
                </header>
            </div>
        }
    }
}
