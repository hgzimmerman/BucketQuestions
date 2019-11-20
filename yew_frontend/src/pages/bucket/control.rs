use crate::pages::bucket::{BucketPage, Msg};
use yewtil::fetch::{FetchState, fetch_to_state_msg};
use wire::bucket::Bucket;
use yew::{ShouldRender, Component, ComponentLink};
use yewtil::NeqAssign;
use wire::question::{Question, NewQuestionRequest};
use uuid::Uuid;
use crate::requests::question::{GetRandomQuestion, DeleteQuestion, CreateQuestion, GetNumberOfQeustionsInTheBucket};
use crate::requests::answer::CreateAnswer;
use wire::answer::NewAnswerRequest;
use yew_router::unit_state::{RouteAgentDispatcher, Route};
use crate::AppRoute;
use yew_router::agent::RouteRequest;
use crate::requests::bucket::{GetPermissionsForUser, AddSelfToBucket};


// TODO consolidate these actions into related groups and define a sub-Msg types for them.
// This will allow simplifying the Bucket data structure, as well as the currently big Msg.
// It will allow breaking this file further into smaller - related chunks to make it easier to find related functionality.
// View might be consolidated in this process.


impl BucketPage {
    pub fn handle_fetched_bucket(&mut self, state: FetchState<Bucket>) -> ShouldRender {
        let rerender = self.bucket.neq_assign(state);

        // get permissions and number of questions.
//                self.link.send_back_batch(|_| vec![Msg::GetPermissions, Msg::GetNumQuestionsInBucket]).emit(());
        // TODO, we need send_batch
        self.link.send_self(Msg::GetPermissions);
        self.link.send_self(Msg::GetNumQuestionsInBucket);

        rerender
    }

    pub fn handle_fetched_active_question(&mut self, state: FetchState<Option<Question>>) -> ShouldRender {
        let rerender = self.active_question.neq_assign(state);
        self.link.send_self(Msg::GetNumQuestionsInBucket);
        rerender
    }

    pub fn handle_post_new_question(&mut self, new_question: FetchState<()>) -> ShouldRender {
        // Since a new question has been created, it is a good idea to also update the number of questions.
        self.link.send_self(Msg::GetNumQuestionsInBucket);
        if let FetchState::Success(question) = new_question {
            self.new_question = "".to_string();
        } else {
            // Notify the toast agent.
        }
        true
    }

//    pub fn handle_post_new_answer(&mut self, response: FetchState<()>) -> ShouldRender {
//        // Re-get the number of questions in the bucket
//        self.link.send_self(Msg::GetNumQuestionsInBucket);
//        if let FetchState::Success(_) = response {
//            self.new_answer = "".to_string();
//            self.active_question = FetchState::NotFetching;
//        } else {
//            // Send error message to toast agent.
//        }
//        true
//    }

    pub fn get_a_random_question(&mut self) -> ShouldRender {
        self.active_question.set_fetching();
        let request = GetRandomQuestion{bucket_uuid: self.bucket.success().map(|bucket| bucket.uuid).unwrap_or_else(|| Uuid::default())};
        self.link.send_future(fetch_to_state_msg(request, Msg::FetchedActiveQuestion));
        true
    }

    pub fn put_question_in_bucket(&mut self) -> ShouldRender {
        self.active_question = FetchState::NotFetching;
        true
    }

    pub fn discard_question(&mut self) -> ShouldRender {
        // The question won't be able to be drawn from the bucket again.
        let mut should_clear_active_question = false;

        let retval = if let FetchState::Success(Some(question)) = &self.active_question {
            should_clear_active_question = true;
            self.new_answer.upload_state.set_fetching(); // TODO Maybe get its own state to track?
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

    pub fn submit_question(&mut self) -> ShouldRender {
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

//    pub fn submit_answer(&mut self) -> ShouldRender {
//        if !self.new_answer.is_empty()
//            && self.active_question.success().map(|value| value.is_some()).unwrap_or_else(|| false)
//        {
//            self.new_answer_create.set_fetching();
//            let request = CreateAnswer(NewAnswerRequest {
//                question_uuid: self.active_question.as_ref().unwrap().as_ref().unwrap().uuid,
//                publicly_visible: true,
//                answer_text: self.new_answer.clone(),
//                archive_question: true
//            });
//            self.link.send_future(fetch_to_state_msg(request, |resp| Msg::FetchedNewAnswerCreate(resp.map(|_| ()))));
//            true
//        } else {
//            true
//        }
//    }

    pub fn show_settings_modal(&mut self) -> ShouldRender {
        let route = AppRoute::BucketSettings{ slug: self.props.slug.clone() };
        RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute( Route::from(route)));
        false
    }

    pub fn get_user_permissions(&mut self) -> ShouldRender {
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

    pub fn get_num_questions_in_bucket(&mut self) -> ShouldRender {
        if let FetchState::Success(bucket) = &self.bucket {
            let request = GetNumberOfQeustionsInTheBucket{ bucket_uuid: bucket.uuid};
            self.link.send_future(fetch_to_state_msg(request, Msg::FetchedNumQuestionsInBucket));
            true
        } else {
            log::warn!("Did not have bucket to use in fetching num questions.");
            false
        }
    }

    pub fn fetched_discarded_question(&mut self, question: FetchState<Question>) -> ShouldRender {
        log::info!("Discarded question: {:?}", question);
        self.link.send_self(Msg::GetNumQuestionsInBucket);
        false
    }

//    pub fn join_bucket(&mut self) -> ShouldRender {
//        if let FetchState::Success(bucket) = &self.bucket {
//            let request = AddSelfToBucket { bucket_uuid: bucket.uuid };
//            self.link.send_future(fetch_to_state_msg(request, |resp|Msg::JoinedBucket(resp.map(|_|()))));
//            true
//        } else {
//            false
//        }
//    }
//
//    pub fn joined_bucket(&mut self) -> ShouldRender {
//        self.update(Msg::GetPermissions);
//        self.update(Msg::GetNumQuestionsInBucket);
//        true
//    }
}

pub mod join {
    use super::*;
    pub struct Join;

    pub enum JoinAction {
        Join,
        Joined
    }

    impl Join {
        fn join_bucket(link: &mut ComponentLink<BucketPage>, bucket: &FetchState<Bucket>) -> ShouldRender {
            if let FetchState::Success(bucket) = &bucket {
                let request = AddSelfToBucket { bucket_uuid: bucket.uuid };
                link.send_future(fetch_to_state_msg(request, |_resp| Msg::Joining(JoinAction::Joined)));
                true
            } else {
                false
            }
        }

        fn joined_bucket(link: &mut ComponentLink<BucketPage>) -> ShouldRender {
            link.send_self(Msg::GetPermissions);
            link.send_self(Msg::GetNumQuestionsInBucket);
            false
        }

        pub fn update(action: JoinAction, link: &mut ComponentLink<BucketPage>, bucket: &FetchState<Bucket>) -> ShouldRender {
            match action {
                JoinAction::Join => Join::join_bucket(link, bucket),
                JoinAction::Joined => Join::joined_bucket(link)
            }
        }
    }
}

pub mod answer {
    use super::*;

    pub enum AnswerAction {
        SubmitNewAnswer,
        FetchedNewAnswerCreate(FetchState<()>),
        UpdateNewAnswer(String),
    }

    #[derive(Default, Debug)]
    pub struct Answer {
        pub upload_state: FetchState<()>,
        pub new_answer_string: String
    }

    impl Answer {
        pub fn update(&mut self, action: AnswerAction, link: &mut ComponentLink<BucketPage>, active_question: &mut FetchState<Option<Question>>) -> ShouldRender {
            match action {
                AnswerAction::SubmitNewAnswer => self.submit_answer(link, active_question),
                AnswerAction::FetchedNewAnswerCreate(response) => self.fetched_new_answer_submission(link, response, active_question),
                AnswerAction::UpdateNewAnswer(answer_text) => {self.update_new_answer(answer_text)}
            }
        }

        pub fn submit_answer(&mut self, link: &mut ComponentLink<BucketPage>, active_question: &FetchState<Option<Question>>) -> ShouldRender {
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

        pub fn fetched_new_answer_submission(&mut self, link: &mut ComponentLink<BucketPage>, response: FetchState<()>, active_question: &mut FetchState<Option<Question>>) -> ShouldRender {
            // Re-get the number of questions in the bucket
            link.send_self(Msg::GetNumQuestionsInBucket);
            if let FetchState::Success(_) = response {
                self.new_answer_string = "".to_string();
                *active_question = FetchState::NotFetching;
            } else {
                // Send error message to toast agent.
            }
            true
        }

        pub fn update_new_answer(&mut self, answer_text: String) -> ShouldRender {
            self.new_answer_string.neq_assign(answer_text)
        }
    }

}