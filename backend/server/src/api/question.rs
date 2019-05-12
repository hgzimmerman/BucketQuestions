use crate::{
    error::Error,
    server_auth::{optional_user_filter, user_filter},
    state::State,
    util::{json_body_filter, json_or_reject},
};
use db::bucket::{
    db_types::{NewFavoriteQuestionRelation, NewQuestion, Question},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{filters::BoxedFilter, path, query, Filter, Reply};
use db::AbstractRepository;

pub const QUESTION_PATH: &str = "question";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BucketUuidQueryParam {
    pub bucket_uuid: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewQuestionRequest {
    /// The bucket to which the question belongs.
    pub bucket_uuid: Uuid,
    /// The content of the question.
    pub question_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetArchivedRequest {
    question_uuid: Uuid,
    archived: bool,
}

pub fn question_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    // impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    let create_question = warp::path::end()
        .and(warp::post2())
        .and(json_body_filter(10))
        .and(optional_user_filter(state))
        .and(state.db2())
        .map(
            |request: NewQuestionRequest,
             user_uuid: Option<Uuid>,
             conn: AbstractRepository|
             -> Result<Question, Error> {
                let new_question = NewQuestion {
                    bucket_uuid: request.bucket_uuid,
                    user_uuid,
                    question_text: request.question_text,
                };
                conn.create_question(new_question).map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    // TODO need a modify question endpoint

    // TODO Not sure of the value of this endpoint
    let delete_question = path!(Uuid)
        .and(warp::path::end())
        .and(warp::delete2())
        .and(state.db2())
        .map(
            |question_uuid: Uuid, conn: AbstractRepository| -> Result<Question, Error> {
                conn.delete_question(question_uuid).map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let random_question = path!("random")
        .and(warp::path::end())
        .and(warp::get2())
        .and(query())
        .and(state.db2())
        .map(
            |query: BucketUuidQueryParam, conn: AbstractRepository| -> Result<Option<Question>, Error> {
                conn.get_random_question(query.bucket_uuid)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let num_questions_in_bucket = path!("number")
        .and(warp::path::end())
        .and(warp::get2())
        .and(query())
        .and(state.db2())
        .map(
            |query: BucketUuidQueryParam, conn: AbstractRepository| -> Result<i64, Error> {
                conn.get_number_of_active_questions_for_bucket(query.bucket_uuid)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let all_questions_in_bucket = path!("in_bucket")
        .and(warp::path::end())
        .and(warp::get2())
        .and(query())
        .and(state.db2())
        .map(
            |query: BucketUuidQueryParam, conn: AbstractRepository| -> Result<Vec<Question>, Error> {
                conn.get_all_questions_for_bucket_of_given_archived_status(query.bucket_uuid, false)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let all_questions_on_floor = path!("on_floor")
        .and(warp::path::end())
        .and(warp::get2())
        .and(query())
        .and(state.db2())
        .map(
            |query: BucketUuidQueryParam, conn: AbstractRepository| -> Result<Vec<Question>, Error> {
                conn.get_all_questions_for_bucket_of_given_archived_status(query.bucket_uuid, true)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    // TODO, this may make sense to remove, or constrain to only putting the question back in the bucket.
    let set_question_archived_state = path!("archive")
        .and(warp::path::end())
        .and(warp::put2())
        .and(json_body_filter(2))
        .and(state.db2())
        .map(
            |request: SetArchivedRequest, conn: AbstractRepository| -> Result<Question, Error> {
                conn.set_archive_status_for_question(request.question_uuid, request.archived)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let favorite_question = path!(Uuid / "favorite")
        .and(warp::path::end())
        .and(warp::post2())
        .and(user_filter(state))
        .and(state.db2())
        .map(|question_uuid: Uuid, user_uuid: Uuid, conn: AbstractRepository| {
            let relation = NewFavoriteQuestionRelation {
                user_uuid,
                question_uuid,
            };
            conn.favorite_question(relation).map_err(Error::from)
        })
        .and_then(json_or_reject);

    let unfavorite_question = path!(Uuid / "favorite")
        .and(warp::path::end())
        .and(warp::delete2())
        .and(user_filter(state))
        .and(state.db2())
        .map(|question_uuid: Uuid, user_uuid: Uuid, conn: AbstractRepository| {
            let relation = NewFavoriteQuestionRelation {
                user_uuid,
                question_uuid,
            };
            conn.unfavorite_question(relation).map_err(Error::from)
        })
        .and_then(json_or_reject);

    let get_favorite_questions = path!("favorites")
        .and(warp::path::end())
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db2())
        .map(|user_uuid: Uuid, conn: AbstractRepository| {
            conn.get_favorite_questions(user_uuid).map_err(Error::from)
        })
        .and_then(json_or_reject);

    // TODO get answers for question: api/question/<uuid>/answers

    path(QUESTION_PATH)
        .and(
            random_question
                .or(create_question)
                .or(delete_question)
                .or(num_questions_in_bucket)
                .or(all_questions_in_bucket)
                .or(all_questions_on_floor)
                .or(set_question_archived_state)
                .or(favorite_question)
                .or(unfavorite_question)
                .or(get_favorite_questions),
        )
        .boxed()
}
