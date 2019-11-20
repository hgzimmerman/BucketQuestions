use crate::pages::bucket::answer::{AnswerState, AnswerAction};
use yew::{Html, html, ShouldRender};
use wire::question::Question;
use yewtil::fetch::{FetchState, fetch_to_state_msg};
use crate::pages::bucket::{BucketLink, BucketPage, Msg};
use uuid::Uuid;
use yewtil::NeqAssign;
use crate::pages::bucket::num_questions::NumQuestionAction;
use crate::requests::question::{GetRandomQuestion, DeleteQuestion};

pub enum ActiveQuestionAction {
    Discard,
    Discarded(FetchState<Question>),
    GetRandom,
    GotRandom(FetchState<Option<Question>>),
    PutBackInBucket
}

#[derive(Debug, Default)]
pub struct ActiveQuestionState(pub FetchState<Option<Question>>);

impl ActiveQuestionState {

    pub fn update(&mut self, action: ActiveQuestionAction, link: &mut BucketLink, bucket_uuid: Option<Uuid>) -> ShouldRender {
        match action {
            ActiveQuestionAction::Discard => self.discard_question(link),
            ActiveQuestionAction::Discarded(question) => Self::fetched_discarded_question(link, question),
            ActiveQuestionAction::GetRandom => self.get_a_random_question(link, bucket_uuid),
            ActiveQuestionAction::GotRandom(question) => self.handle_fetched_active_question(link, question),
            ActiveQuestionAction::PutBackInBucket => self.put_question_in_bucket(),
        }
    }

    pub fn render_q_and_a_card(&self, new_answer: &AnswerState) -> Html<BucketPage> {
        let content = match &self.0 {
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
                        onclick=|_| Msg::ActiveQuestion(ActiveQuestionAction::GetRandom)
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
                            <button class="button is-info" onclick = |_| Msg::ActiveQuestion(ActiveQuestionAction::PutBackInBucket)>
                                {"Put Back"}
                            </button>
                            <button class="button is-warning" onclick = |_| Msg::ActiveQuestion(ActiveQuestionAction::Discard)>
                                {"Discard"}
                            </button>
                        </div>

                        <textarea
                            class = "textarea is-medium"
                            rows=6
                            value=&new_answer.new_answer_string
                            oninput=|e| Msg::Answer(AnswerAction::UpdateNewAnswer(e.value))
                            placeholder="Answer"
                        />
                    </div>

                    <div class="card-footer">
                        <button
                            class= "button is-success card-footer-item is-radiusless"
                            onclick= |_| Msg::Answer(AnswerAction::SubmitNewAnswer)
                            disabled=new_answer.new_answer_string.is_empty()
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
                            onclick=|_| Msg::ActiveQuestion(ActiveQuestionAction::GetRandom)
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

        let title = match &self.0 {
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

    fn discard_question(&mut self, link: &mut BucketLink) -> ShouldRender {
        // The question won't be able to be drawn from the bucket again.
        let mut should_clear_active_question = false;

        let retval = if let FetchState::Success(Some(question)) = &self.0 {
            should_clear_active_question = true;
//                answer_state.new_answer.upload_state.set_fetching(); // TODO Maybe get its own state to track?
            let request = DeleteQuestion {
                question_uuid: question.uuid
            };
            link.send_future(fetch_to_state_msg(request, |resp| Msg::ActiveQuestion(ActiveQuestionAction::Discarded(resp))));
            true
        } else {
            true
        };

        if should_clear_active_question {
            self.0 = FetchState::NotFetching;
        }

        retval
    }

    fn put_question_in_bucket(&mut self) -> ShouldRender {
        self.0 = FetchState::NotFetching;
        true
    }

    fn handle_fetched_active_question(&mut self, link: &mut BucketLink, state: FetchState<Option<Question>>) -> ShouldRender {
        let rerender = self.0.neq_assign(state);
        link.send_self(Msg::NumQuestions(NumQuestionAction::Get));
        rerender
    }

    fn get_a_random_question(&mut self, link: &mut BucketLink, bucket_uuid: Option<Uuid>) -> ShouldRender {
        if let Some(bucket_uuid) = bucket_uuid {
            self.0.set_fetching();
            let request = GetRandomQuestion{bucket_uuid};
            link.send_future(fetch_to_state_msg(request, |resp| Msg::ActiveQuestion(ActiveQuestionAction::GotRandom(resp))));
            true
        } else {
            // TODO send error to toast agent
            false
        }
    }



    fn fetched_discarded_question(link: &mut BucketLink, question: FetchState<Question>) -> ShouldRender {
        log::info!("Discarded question: {:?}", question);
        link.send_self(Msg::NumQuestions(NumQuestionAction::Get));
        false
    }
}

