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
use crate::pages::bucket::control::num_questions::NumQuestionAction;
use crate::pages::bucket::control::permissions::PermissionsAction;


// TODO consolidate these actions into related groups and define a sub-Msg types for them.
// This will allow simplifying the Bucket data structure, as well as the currently big Msg.
// It will allow breaking this file further into smaller - related chunks to make it easier to find related functionality.
// View might be consolidated in this process.

type BucketLink = ComponentLink<BucketPage>;

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

pub mod join {
    use super::*;
    use crate::pages::bucket::control::num_questions::NumQuestionAction;
    use crate::pages::bucket::control::permissions::PermissionsAction;

    pub struct JoinLogic;

    pub enum JoinAction {
        Join,
        Joined
    }

    impl JoinLogic {
        fn join_bucket(link: &mut BucketLink, bucket: &FetchState<Bucket>) -> ShouldRender {
            if let FetchState::Success(bucket) = &bucket {
                let request = AddSelfToBucket { bucket_uuid: bucket.uuid };
                link.send_future(fetch_to_state_msg(request, |_resp| Msg::Joining(JoinAction::Joined)));
                true
            } else {
                false
            }
        }

        fn joined_bucket(link: &mut BucketLink) -> ShouldRender {
            link.send_self(Msg::Permissions(PermissionsAction::Get));
            link.send_self(Msg::NumQuestions(NumQuestionAction::Get));
            false
        }

        pub fn update(action: JoinAction, link: &mut BucketLink, bucket: &FetchState<Bucket>) -> ShouldRender {
            match action {
                JoinAction::Join => JoinLogic::join_bucket(link, bucket),
                JoinAction::Joined => JoinLogic::joined_bucket(link)
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

}

pub mod new_question {
    use super::*;
    use crate::pages::bucket::control::num_questions::NumQuestionAction;

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

        pub fn update(&mut self, action: NewQuestionAction, link: &mut ComponentLink<BucketPage>, bucket: &FetchState<Bucket>) -> ShouldRender {
            match action {
                NewQuestionAction::Submit => self.submit_new_question(link, bucket),
                NewQuestionAction::HandleSubmission(response) => self.fetched_new_question_create(link, response),
                NewQuestionAction::UpdateText(question_text) => self.update_new_question_text(question_text),
            }
        }

        fn submit_new_question(&mut self, link: &mut ComponentLink<BucketPage>, bucket: &FetchState<Bucket>) -> ShouldRender {
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
}

pub mod num_questions {
    use super::*;

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
}

pub mod permissions {
    use super::*;
    use wire::bucket_user_relation::BucketUserPermissions;

    pub enum PermissionsAction {
        Get,
        Fetched(FetchState<BucketUserPermissions>),
    }

    #[derive(Debug, Default)]
    pub struct PermissionsState {
        pub permissions: FetchState<BucketUserPermissions>
    }

    impl PermissionsState {
        pub fn update(&mut self, action: PermissionsAction, link: &mut BucketLink, bucket_uuid: Option<Uuid>) -> ShouldRender {
            match action {
                PermissionsAction::Get => self.get_user_permissions(link, bucket_uuid),
                PermissionsAction::Fetched(permissions) => self.permissions.neq_assign(permissions)
            }
        }

        fn get_user_permissions(&mut self, link: &mut BucketLink, bucket_uuid: Option<Uuid>) -> ShouldRender {
            log::info!("getting permissions");
            if let Some(bucket_uuid) = bucket_uuid {
                self.permissions.set_fetching();
                let request = GetPermissionsForUser{ bucket_uuid};
                link.send_future(fetch_to_state_msg(request, |resp| Msg::Permissions(PermissionsAction::Fetched(resp))));
                true
            } else {
                log::warn!("Did not have bucket to use in fetching permissions.");
                false
            }
        }
    }
}

pub mod active_question {
    use super::*;
    use crate::pages::bucket::control::answer::AnswerState;

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



        fn discard_question(&mut self, link: &mut BucketLink) -> ShouldRender {
            // The question won't be able to be drawn from the bucket again.
            let mut should_clear_active_question = false;

            let retval = if let FetchState::Success(Some(question)) = &self.0 {
                should_clear_active_question = true;
//                answer_state.new_answer.upload_state.set_fetching(); // TODO Maybe get its own state to track?
                let request = DeleteQuestion{
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


}