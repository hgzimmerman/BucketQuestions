use yewtil::fetch::{FetchState, fetch_to_state_msg};
use crate::pages::bucket::{BucketLink, Msg};
use yew::ShouldRender;
use wire::question::Question;
use crate::requests::answer::CreateAnswer;
use wire::answer::NewAnswerRequest;
use crate::pages::bucket::num_questions::NumQuestionAction;
use yewtil::NeqAssign;

pub enum AnswerAction {
    SubmitNewAnswer,
    FetchedNewAnswerCreate(FetchState<()>),
    UpdateNewAnswer(String),
}

#[derive(Default, Debug)]
pub struct AnswerState {
    pub upload_state: FetchState<()>,
    pub new_answer_string: String
}

impl AnswerState {
    pub fn update(&mut self, action: AnswerAction, link: &mut BucketLink, active_question: &mut FetchState<Option<Question>>) -> ShouldRender {
        match action {
            AnswerAction::SubmitNewAnswer => self.submit_answer(link, active_question),
            AnswerAction::FetchedNewAnswerCreate(response) => self.fetched_new_answer_submission(link, response, active_question),
            AnswerAction::UpdateNewAnswer(answer_text) => {self.update_new_answer(answer_text)}
        }
    }

    fn submit_answer(&mut self, link: &mut BucketLink, active_question: &FetchState<Option<Question>>) -> ShouldRender {
        if !self.new_answer_string.is_empty()
            && active_question.success().map(|value| value.is_some()).unwrap_or_else(|| false)
        {
            self.upload_state.set_fetching();
            let request = CreateAnswer(NewAnswerRequest {
                question_uuid: active_question.as_ref().unwrap().as_ref().unwrap().uuid, // TODO better error handling here. use if let above ^^
                publicly_visible: true,
                answer_text: self.new_answer_string.clone(),
                archive_question: true
            });
            link.send_future(fetch_to_state_msg(request, |resp| Msg::Answer(AnswerAction::FetchedNewAnswerCreate(resp.map(|_| ())))));
            true
        } else {
            true
        }
    }

    fn fetched_new_answer_submission(&mut self, link: &mut BucketLink, response: FetchState<()>, active_question: &mut FetchState<Option<Question>>) -> ShouldRender {
        // Re-get the number of questions in the bucket
        link.send_self(Msg::NumQuestions(NumQuestionAction::Get));
        if let FetchState::Success(_) = response {
            self.new_answer_string = "".to_string();
            *active_question = FetchState::NotFetching;
        } else {
            // Send error message to toast agent.
        }
        true
    }

    fn update_new_answer(&mut self, answer_text: String) -> ShouldRender {
        self.new_answer_string.neq_assign(answer_text)
    }
}
