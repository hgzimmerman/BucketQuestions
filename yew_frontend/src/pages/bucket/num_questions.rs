use yewtil::fetch::{FetchState, fetch_to_state_msg};
use crate::pages::bucket::{BucketLink, Msg};
use uuid::Uuid;
use yew::ShouldRender;
use yewtil::NeqAssign;
use crate::requests::question::GetNumberOfQeustionsInTheBucket;

pub enum NumQuestionAction {
    Get,
    Handle(FetchState<usize>)
}

#[derive(Default, Debug)]
pub struct NumQuestionsState {
    pub num_questions: FetchState<usize>
}

impl NumQuestionsState {
    pub fn update(&mut self, action: NumQuestionAction, link: &mut BucketLink, bucket_uuid: Option<Uuid>) -> ShouldRender {
        match action {
            NumQuestionAction::Get => self.get_num_questions_in_bucket(link, bucket_uuid),
            NumQuestionAction::Handle(response) => {
                log::info!("number: {:?}", response);
                self.num_questions.neq_assign(response)
            },
        }
    }

    fn get_num_questions_in_bucket(&mut self, link: &mut BucketLink, bucket_uuid: Option<Uuid>) -> ShouldRender {
        if let Some(bucket_uuid) = bucket_uuid {
            let request = GetNumberOfQeustionsInTheBucket{ bucket_uuid};
            link.send_future(fetch_to_state_msg(request, |resp| Msg::NumQuestions(NumQuestionAction::Handle(resp))));
            true
        } else {
            log::warn!("Did not have bucket to use in fetching num questions.");
            false
        }
    }

}
