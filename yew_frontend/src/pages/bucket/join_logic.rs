use crate::pages::bucket::num_questions::NumQuestionAction;
use crate::pages::bucket::permissions::PermissionsAction;
use crate::pages::bucket::{BucketLink, Msg};
use yewtil::fetch::{FetchState, fetch_to_state_msg};
use crate::requests::bucket::AddSelfToBucket;
use yew::ShouldRender;
use wire::bucket::Bucket;

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

