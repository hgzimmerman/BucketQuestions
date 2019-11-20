use crate::pages::bucket::num_questions::NumQuestionAction;
use yew::{Html, html, ShouldRender, ComponentLink};
use yewtil::fetch::{FetchState, fetch_to_state_msg};
use crate::pages::bucket::{BucketPage, Msg, BucketLink};
use crate::requests::question::CreateQuestion;
use wire::bucket::Bucket;
use wire::question::NewQuestionRequest;
use yewtil::NeqAssign;

pub enum NewQuestionAction {
    UpdateText(String),
    Submit,
    HandleSubmission(FetchState<()>)
}

#[derive(Default, Debug)]
pub struct NewQuestionState {
    pub upload_state: FetchState<()>,
    pub new_question_text: String
}

impl NewQuestionState {

    pub fn render_new_question_card(&self) -> Html<BucketPage> {
        let textarea: Html<BucketPage> = match &self.upload_state {
            FetchState::Success(_)
            | FetchState::NotFetching => html! {
                <textarea
                    class = "textarea is-medium"
                    rows=6
                    value=&self.new_question_text
                    oninput=|e| Msg::NewQuestion(NewQuestionAction::UpdateText(e.value))
                    placeholder="New Question"
                />
            },
            FetchState::Fetching => html! {
                <textarea
                    class = "textarea is-medium is-loading"
                    rows=6
                    value=&self.new_question_text
                    oninput=|e| Msg::NewQuestion(NewQuestionAction::UpdateText(e.value))
                    placeholder="New Question"
                />
            },
            FetchState::Failed(_) => html! {
                <textarea
                    class = "textarea is-medium is-danger"
                    rows=6
                    value=&self.new_question_text
                    oninput=|e| Msg::NewQuestion(NewQuestionAction::UpdateText(e.value))
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
                        onclick=|_| Msg::NewQuestion(NewQuestionAction::Submit)
                        disabled=self.new_question_text.is_empty()
                    >
                         {"Submit New Question"}
                    </button>
                </div>
            </div>
        }
    }
    pub fn update(&mut self, action: NewQuestionAction, link: &mut BucketLink, bucket: &FetchState<Bucket>) -> ShouldRender {
        match action {
            NewQuestionAction::Submit => self.submit_new_question(link, bucket),
            NewQuestionAction::HandleSubmission(response) => self.fetched_new_question_create(link, response),
            NewQuestionAction::UpdateText(question_text) => self.update_new_question_text(question_text),
        }
    }

    fn submit_new_question(&mut self, link: &mut BucketLink, bucket: &FetchState<Bucket>) -> ShouldRender {
        if !self.new_question_text.is_empty()
            && bucket.success().is_some()
        {
            self.upload_state.set_fetching();
            let request = CreateQuestion{
                new_question: NewQuestionRequest {
                    bucket_uuid: bucket.as_ref().unwrap().uuid,
                    question_text: self.new_question_text.clone()
                }
            };
            link.send_future(fetch_to_state_msg(request, |resp| Msg::NewQuestion(NewQuestionAction::HandleSubmission(resp.map(|_|())))));
            true
        } else {
            log::warn!("Tried to add a new question when the question was empty, or the bucket uuid was unknown.");
            false
        }
    }

    fn fetched_new_question_create(&mut self, link: &mut BucketLink, new_question: FetchState<()>) -> ShouldRender {
        // Since a new question has been created, it is a good idea to also update the number of questions.
        link.send_self(Msg::NumQuestions(NumQuestionAction::Get));
        if let FetchState::Success(question) = new_question {
            self.new_question_text = "".to_string();
        } else {
            // Notify the toast agent.
        }
        true
    }


    fn update_new_question_text(&mut self, question_text: String) -> ShouldRender {
        self.new_question_text.neq_assign(question_text)
    }
}