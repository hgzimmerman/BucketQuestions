use crate::{
    error::Error,
    server_auth::{optional_user_filter, user_filter},
    state::State,
    util::{json_or_reject, sized_body_json},
};
use db::{
    favorite_question::db_types::NewFavoriteQuestionRelation,
    question::db_types::{NewQuestion, Question},
    BoxedRepository,
};
use uuid::Uuid;
use warp::{filters::BoxedFilter, path, query, Filter, Reply};

pub const QUESTION_PATH: &str = "question";

use wire::question::{NewQuestionRequest, BucketUuidQueryParam, SetArchivedRequest};

pub fn question_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    // impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    let create_question = warp::path::end()
        .and(warp::post2())
        .and(sized_body_json(10))
        .and(optional_user_filter(state))
        .and(state.db())
        .map(
            |request: NewQuestionRequest,
             user_uuid: Option<Uuid>,
             conn: BoxedRepository|
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
    // At the very least, you should be able to only delete questions that you either
    // * created yourself
    // * or have a permission to delete.
    let delete_question = path!(Uuid)
        .and(warp::path::end())
        .and(warp::delete2())
        .and(state.db())
        .map(
            |question_uuid: Uuid, conn: BoxedRepository| -> Result<Question, Error> {
                conn.delete_question(question_uuid).map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let random_question = path!("random")
        .and(warp::path::end())
        .and(warp::get2())
        .and(query())
        .and(state.db())
        .map(
            |query: BucketUuidQueryParam,
             conn: BoxedRepository|
             -> Result<Option<Question>, Error> {
                conn.get_random_question(query.bucket_uuid)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let num_questions_in_bucket = path!("number")
        .and(warp::path::end())
        .and(warp::get2())
        .and(query())
        .and(state.db())
        .map(
            |query: BucketUuidQueryParam, conn: BoxedRepository| -> Result<i64, Error> {
                conn.get_number_of_active_questions_for_bucket(query.bucket_uuid)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let all_questions_in_bucket = path!("in_bucket")
        .and(warp::path::end())
        .and(warp::get2())
        .and(query())
        .and(state.db())
        .map(
            |query: BucketUuidQueryParam, conn: BoxedRepository| -> Result<Vec<Question>, Error> {
                conn.get_all_questions_for_bucket_of_given_archived_status(query.bucket_uuid, false)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let all_questions_on_floor = path!("on_floor")
        .and(warp::path::end())
        .and(warp::get2())
        .and(query())
        .and(state.db())
        .map(
            |query: BucketUuidQueryParam, conn: BoxedRepository| -> Result<Vec<Question>, Error> {
                conn.get_all_questions_for_bucket_of_given_archived_status(query.bucket_uuid, true)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    // TODO, this may make sense to remove, or constrain to only putting the question back in the bucket.
    let set_question_archived_state = path!("archive")
        .and(warp::path::end())
        .and(warp::put2())
        .and(sized_body_json(2))
        .and(state.db())
        .map(
            |request: SetArchivedRequest, conn: BoxedRepository| -> Result<Question, Error> {
                conn.set_archive_status_for_question(request.question_uuid, request.archived)
                    .map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let favorite_question = path!(Uuid / "favorite")
        .and(warp::path::end())
        .and(warp::post2())
        .and(user_filter(state))
        .and(state.db())
        .map(
            |question_uuid: Uuid, user_uuid: Uuid, conn: BoxedRepository| -> Result<(), Error> {
                let relation = NewFavoriteQuestionRelation {
                    user_uuid,
                    question_uuid,
                };
                conn.favorite_question(relation).map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let unfavorite_question = path!(Uuid / "favorite")
        .and(warp::path::end())
        .and(warp::delete2())
        .and(user_filter(state))
        .and(state.db())
        .map(
            |question_uuid: Uuid, user_uuid: Uuid, conn: BoxedRepository| -> Result<(), Error> {
                let relation = NewFavoriteQuestionRelation {
                    user_uuid,
                    question_uuid,
                };
                conn.unfavorite_question(relation).map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let get_favorite_questions = path!("favorites")
        .and(warp::path::end())
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db())
        .map(|user_uuid: Uuid, conn: BoxedRepository| -> Result<Vec<Question>, Error> {
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
