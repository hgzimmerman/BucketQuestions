use yew::{Component, ComponentLink, Properties, html, Html};
use yew::virtual_dom::VNode;
use yewtil::NeqAssign;
use wire::question::{Question, NewQuestionRequest};
use crate::common::{FetchState, fetch_to_state_msg};
use crate::requests::question::{GetRandomQuestion, CreateQuestion, DeleteQuestion};
use crate::pages::bucket::Msg::FetchedActiveQuestion;
use wire::bucket::Bucket;
use crate::requests::bucket::GetBucketBySlug;
use uuid::Uuid;
use crate::requests::answer::CreateAnswer;
use wire::answer::NewAnswerRequest;

pub struct BucketPage {
    props: Props,
    link: ComponentLink<BucketPage>,
    bucket: FetchState<Bucket>,
    active_question: FetchState<Option<Question>>,
    new_question: String,
    new_question_create: FetchState<()>,
    new_answer: String,
    new_answer_create: FetchState<()>,
    questions_in_bucket_count: FetchState<usize>
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub slug: String
}

pub enum Msg {
    FetchedBucket(FetchState<Bucket>),
    FetchedActiveQuestion(FetchState<Option<Question>>),
    UpdateNewQuestion(String),
    FetchedNewQuestionCreate(()),
    UpdateNewAnswer(String),
    FetchedNewAnswerCreate(()),
    GetARandomQuestion,
    PutQuestionBackInBucket,
    DiscardQuestion,
    SubmitNewQuestion,
    SubmitNewAnswer
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
            questions_in_bucket_count: Default::default()
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
            Msg::FetchedBucket(state) => {self.bucket.neq_assign(state)}
            Msg::FetchedActiveQuestion(state) => self.active_question.neq_assign(state),
            Msg::UpdateNewQuestion(question_text) => self.new_question.neq_assign(question_text),
            Msg::FetchedNewQuestionCreate(_) => {
                // TODO if success, then ...
                self.new_question = "".to_string();
                true
            }
            Msg::UpdateNewAnswer(answer_text) => {
                self.new_answer.neq_assign(answer_text)
            }
            Msg::FetchedNewAnswerCreate(_) => {

                // TODO if success, then ...
                self.new_answer = "".to_string();
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
                    self.link.send_future(fetch_to_state_msg(request, |_| Msg::FetchedNewAnswerCreate(())));
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
                    self.link.send_future(fetch_to_state_msg(request, |_| Msg::FetchedNewQuestionCreate(())));
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
                        answer_text: self.new_answer.clone()
                    });
                    self.link.send_future(fetch_to_state_msg(request, |_| Msg::FetchedNewAnswerCreate(())));
                    true
                } else {
                    true
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html<Self> {
        html! {

            <div class= "has-background-primary full_height_scrollable">
                <div class = "container" >
                    {self.render_q_and_a_card()}
                    {self.render_new_question_card()}
                </div>
            </div>
        }
    }
}

impl BucketPage {

    fn render_q_and_a_card(&self) -> Html<Self> {
        let content = match &self.active_question {
            FetchState::Fetching => html! {
                <>
                    <button
                        class = "button is-success is-loading is-fullwidth"
                        disabled = true
                    >
                        {"Get A Random Question"}
                    </button>
                </>
            },
            FetchState::NotFetching => html! {
                <button
                    class = "button is-success is-fullwidth"
                    onclick=|_| Msg::GetARandomQuestion
                >
                    {"Get A Random Question"}
                </button>
            },
            FetchState::Success(Some(question)) => html! {
                <>
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
                        placeholder="Answer (can be left blank)"
                    />

                    <button class= "button is-success is-fullwidth" onclick= |_| Msg::SubmitNewAnswer>
                        {"Answer"}
                    </button>

                </>
            },
            FetchState::Success(None) => html! {
                html! {
                    <>
                        <div>
                            {"No questions in this bucket. Try adding some!"}
                        </div>
                        <button
                            class = "button is-success is-fullwidth"
                            onclick=|_| Msg::GetARandomQuestion
                        >
                            {"Get A Random Question"}
                        </button>
                    </>
                }
            },
            FetchState::Failed(_) => html! {
                {"Something went wrong :("}
            }
        };
        html!{
            <div class = "box ">
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
            <div class = "box">
                {textarea}
                <button class= "button is-success is-fullwidth" onclick=|_| Msg::SubmitNewQuestion>
                     {"Submit New Question"}
                </button>
            </div>
        }
    }
}