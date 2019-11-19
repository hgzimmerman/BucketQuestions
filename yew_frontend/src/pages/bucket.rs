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

// TODO break this component across a data-wiring, view and update modules. @ 500 lines, this is getting hard to follow.
pub struct BucketPage {
    props: Props,
    link: ComponentLink<BucketPage>,
    bucket: FetchState<Bucket>,
    active_question: FetchState<Option<Question>>,
    new_question: String,
    new_question_create: FetchState<()>,
    new_answer: String,
    new_answer_create: FetchState<()>,
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
    UpdateNewAnswer(String),
    FetchedNewAnswerCreate(FetchState<()>),
    GetARandomQuestion,
    PutQuestionBackInBucket,
    GetPermissions,
    FetchedPermissions(FetchState<BucketUserPermissions>),
    GetNumQuestionsInBucket,
    FetchedNumQuestionsInBucket(FetchState<usize>),
    DiscardQuestion,
    FetchedDiscardQuestion(FetchState<Question>),
    SubmitNewQuestion,
    SubmitNewAnswer,
    ShowSettingsModal,
    JoinBucket,
    JoinedBucket(FetchState<()>)
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
            new_answer: "".to_string(),
            new_answer_create: Default::default(),
            questions_in_bucket_count: Default::default(),
            permissions: Default::default()
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
            Msg::FetchedBucket(state) => {
                let rerender = self.bucket.neq_assign(state);

                // get permissions and number of questions.
//                self.link.send_back_batch(|_| vec![Msg::GetPermissions, Msg::GetNumQuestionsInBucket]).emit(());
                // TODO, we need send_batch
                self.link.send_self(Msg::GetPermissions);
                self.link.send_self(Msg::GetNumQuestionsInBucket);

                rerender
            }
            Msg::FetchedActiveQuestion(state) => {
                let rerender = self.active_question.neq_assign(state);
                self.link.send_self(Msg::GetNumQuestionsInBucket);
                rerender
            },
            Msg::UpdateNewQuestion(question_text) => self.new_question.neq_assign(question_text),
            Msg::FetchedNewQuestionCreate(new_question) => {
                self.link.send_self(Msg::GetNumQuestionsInBucket);
                // TODO if success, then ...
                if let FetchState::Success(question) = new_question {
                    self.new_question = "".to_string();
                } else {
                    // Notify the toast agent.
                }
                true
            }
            Msg::UpdateNewAnswer(answer_text) => {
                self.new_answer.neq_assign(answer_text)
            }
            Msg::FetchedNewAnswerCreate(response) => {
                // Re-get the number of questions in the bucket
                self.link.send_self(Msg::GetNumQuestionsInBucket);
                // TODO if success, then ...
                if let FetchState::Success(_) = response {
                    self.new_answer = "".to_string();
                    self.active_question = FetchState::NotFetching;
                } else {
                    // Send error message to toast agent.
                }
                true
            }
            Msg::GetARandomQuestion => {
                self.active_question.set_fetching();
                let request = GetRandomQuestion{bucket_uuid: self.bucket.success().map(|bucket| bucket.uuid).unwrap_or_else(|| Uuid::default())};
                self.link.send_future(fetch_to_state_msg(request, Msg::FetchedActiveQuestion));
                true
            }
            Msg::PutQuestionBackInBucket => {
                self.active_question = FetchState::NotFetching;
                true
            }
            Msg::DiscardQuestion => {
                // The question won't be able to be drawn from the bucket again.
                let mut should_clear_active_question = false;

                let retval = if let FetchState::Success(Some(question)) = &self.active_question {
                    should_clear_active_question = true;
                    self.new_answer_create.set_fetching();
                    let request = DeleteQuestion{
                        question_uuid: question.uuid
                    };
                    self.link.send_future(fetch_to_state_msg(request, Msg::FetchedDiscardQuestion));
                    true
                } else {
                    true
                };

                if should_clear_active_question {
                    self.active_question = FetchState::NotFetching;
                }

                retval
            }
            Msg::SubmitNewQuestion => {
                if !self.new_question.is_empty()
                && self.bucket.success().is_some()
                {
                    self.new_question_create.set_fetching();
                    let request = CreateQuestion{
                        new_question: NewQuestionRequest {
                            bucket_uuid: self.bucket.as_ref().unwrap().uuid,
                            question_text: self.new_question.clone()
                        }
                    };
                    self.link.send_future(fetch_to_state_msg(request, |resp| Msg::FetchedNewQuestionCreate(resp.map(|_|()))));
                    true
                } else {
                    log::warn!("Tried to add a new question when the question was empty, or the bucket uuid was unknown.");
                    false
                }
            }
            Msg::SubmitNewAnswer => {
                if !self.new_answer.is_empty()
                && self.active_question.success().map(|value| value.is_some()).unwrap_or_else(|| false)
                {
                    self.new_answer_create.set_fetching();
                    let request = CreateAnswer(NewAnswerRequest {
                        question_uuid: self.active_question.as_ref().unwrap().as_ref().unwrap().uuid,
                        publicly_visible: true,
                        answer_text: self.new_answer.clone(),
                        archive_question: true
                    });
                    self.link.send_future(fetch_to_state_msg(request, |resp| Msg::FetchedNewAnswerCreate(resp.map(|_| ()))));
                    true
                } else {
                    true
                }
            }
            Msg::ShowSettingsModal => {
                let route = AppRoute::BucketSettings{ slug: self.props.slug.clone() };
                RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute( Route::from(route)));
                false
            }
            Msg::FetchedPermissions(permissions) => {
                self.permissions.neq_assign(permissions)
            }
            Msg::GetPermissions => {
                log::info!("getting permissions");
                if let FetchState::Success(bucket) = &self.bucket {
                    self.permissions.set_fetching();
                    let request = GetPermissionsForUser{ bucket_uuid: bucket.uuid};
                    self.link.send_future(fetch_to_state_msg(request, Msg::FetchedPermissions));
                    true
                } else {
                    log::warn!("Did not have bucket to use in fetching permissions.");
                    false
                }
            }
            Msg::GetNumQuestionsInBucket => {
                if let FetchState::Success(bucket) = &self.bucket {
                    let request = GetNumberOfQeustionsInTheBucket{ bucket_uuid: bucket.uuid};
                    self.link.send_future(fetch_to_state_msg(request, Msg::FetchedNumQuestionsInBucket));
                    true
                } else {
                    log::warn!("Did not have bucket to use in fetching num questions.");
                    false
                }
            }
            Msg::FetchedNumQuestionsInBucket(num) => {
                self.questions_in_bucket_count.neq_assign(num)
            }
            Msg::FetchedDiscardQuestion(question) => {
                log::info!("Discarded question: {:?}", question);
                self.link.send_self(Msg::GetNumQuestionsInBucket);
                false
            }
            Msg::JoinBucket => {
                if let FetchState::Success(bucket) = &self.bucket {
                    let request = AddSelfToBucket { bucket_uuid: bucket.uuid };
                    self.link.send_future(fetch_to_state_msg(request, |resp|Msg::JoinedBucket(resp.map(|_|()))));
                    true
                } else {
                    false
                }
            }
            Msg::JoinedBucket(_) => {
                self.update(Msg::GetPermissions);
                self.update(Msg::GetNumQuestionsInBucket);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                {self.modal()}
                <div class= "has-background-primary full_height_scrollable">
                    <div class = "full_width">
                        <div class = "columns is-centered no_margin">
                            <div class="column is-two-thirds-tablet is-half-desktop is-centered">
                                {self.render_title()}
                                {self.render_q_and_a_card()}
                                {self.render_new_question_card()}
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

enum SettingsJoin {
    Settings,
    JoinBucket
}

impl BucketPage {

    // TODO It may not be wise to rely on a 403/401 error to indicate that a certain button should exist.
    // TODO if it is indeed a 404, that is probably better.
    /// Show settings if the user is capable of editing any of them.
    /// Or, if the permissions can't be gotten, show the option to join the bucket.
    ///
    /// None represents that neither should be rendered
    fn show_settings_or_join_button(&self) -> Option<SettingsJoin> {
        match &self.permissions {
            FetchState::Success(permissions) => {
                if permissions.grant_permissions_permission
                    || permissions.kick_permission
                    || permissions.set_exclusive_permission
                    || permissions.set_drawing_permission
                    || permissions.set_public_permission {
                    Some(SettingsJoin::Settings)
                } else {
                    None
                }
            }
            // TODO proide a more narrow error here.
            FetchState::Failed(_) => {
                Some(SettingsJoin::JoinBucket)
            }
            _ => None
        }
    }

    fn modal(&self) -> Html<Self> {
        if let (FetchState::Success(bucket), FetchState::Success(permissions) )= (&self.bucket, &self.permissions) {
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
        let settings_modal_link_or_join_button = match self.show_settings_or_join_button() {
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
                        onclick=|_| Msg::JoinBucket
                    >
                        {"Join"}
                    </button>
                </div>
            },
            None => html!{}
        } ;

        let num_questions_in_bucket = if let FetchState::Success(count) = self.questions_in_bucket_count {
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

    fn render_q_and_a_card(&self) -> Html<Self> {
        let content = match &self.active_question {
            FetchState::Fetching => html! {
                <div class="card-footer">
                    <button
                        class = "button is-success card-footer-item is-radiusless"
                        disabled = true
                    >
                        {"Get A Random Question"}
                    </button>
                </div>
            },
            FetchState::NotFetching => html! {

                <div class="card-footer">
                    <button
                        class = "button is-success card-footer-item is-radiusless"
                        onclick=|_| Msg::GetARandomQuestion
                    >
                        {"Get A Random Question"}
                    </button>
                </div>
            },
            FetchState::Success(Some(question)) => html! {
                <>
                    <div class="card-content">
                        <div class="is-size-4">
                            <p>{&question.question_text}</p>
                        </div>
                        <br />


                        <div class="level">
                            <button class="button is-info" onclick = |_| Msg::PutQuestionBackInBucket>
                                {"Put Back"}
                            </button>
                            <button class="button is-warning" onclick = |_| Msg::DiscardQuestion>
                                {"Discard"}
                            </button>
                        </div>

                        <textarea
                            class = "textarea is-medium"
                            rows=6
                            value=&self.new_answer
                            oninput=|e| Msg::UpdateNewAnswer(e.value)
                            placeholder="Answer"
                        />
                    </div>

                    <div class="card-footer">
                        <button
                            class= "button is-success card-footer-item is-radiusless"
                            onclick= |_| Msg::SubmitNewAnswer
                            disabled=self.new_answer.is_empty()
                        >
                            {"Answer"}
                        </button>
                    </div>

                </>
            },
            FetchState::Success(None) => html! {
                html! {
                    <>
                        <div class="card-content">
                            {"No questions in this bucket. Try adding some!"}
                        </div>
                        <div class="card-footer">
                            <button
                                class= "button is-success card-footer-item is-radiusless"
                                onclick=|_| Msg::GetARandomQuestion
                            >
                                {"Get A Random Question"}
                            </button>

                        </div>
                    </>
                }
            },
            FetchState::Failed(_) => html! {
                {"Something went wrong :("}
            }
        };

        let title = match &self.active_question {
            FetchState::Success(_) => "Answer Question",
            _ => "Draw Question From Bucket"
        };

        html!{
            <div class="card column_margin">
                <header class="card-header">
                    <p class="card-header-title">
                        {title}
                    </p>
                </header>
                {content}
            </div>
        }
    }

    fn render_new_question_card(&self) -> Html<Self> {
        let textarea: Html<Self> = match &self.new_question_create {
            FetchState::Success(_)
            | FetchState::NotFetching => html! {
                <textarea
                    class = "textarea is-medium"
                    rows=6
                    value=&self.new_question
                    oninput=|e| Msg::UpdateNewQuestion(e.value)
                    placeholder="New Question"
                />
            },
            FetchState::Fetching => html! {
                <textarea
                    class = "textarea is-medium is-loading"
                    rows=6
                    value=&self.new_question
                    oninput=|e| Msg::UpdateNewQuestion(e.value)
                    placeholder="New Question"
                />
            },
            FetchState::Failed(_) => html! {
                <textarea
                    class = "textarea is-medium is-danger"
                    rows=6
                    value=&self.new_question
                    oninput=|e| Msg::UpdateNewQuestion(e.value)
                    placeholder="New Question"
                />
            }
        };

        html! {
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">
                        {"New Question"}
                    </p>
                </header>
                <div class="card-content">
                    {textarea}
                </div>
                <div class="card-footer">
                    <button
                        class= "button is-success card-footer-item is-radiusless"
                        onclick=|_| Msg::SubmitNewQuestion
                        disabled=self.new_question.is_empty()
                    >
                         {"Submit New Question"}
                    </button>
                </div>
            </div>
        }
    }
}