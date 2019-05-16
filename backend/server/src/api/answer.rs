use crate::{
    error::Error,
    server_auth::optional_user_filter,
    state::State,
    util::{json_or_reject, sized_body_json},
};
use db::{
    answer::db_types::{Answer, NewAnswer},
    BoxedRepository,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{filters::BoxedFilter, path, Filter, Reply};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnswerRequest {
    /// The question to which the answer is responding.
    pub question_uuid: Uuid,
    /// Can the outside world see the answer.
    pub publicly_visible: bool,
    /// The answer
    pub answer_text: String,
}

pub const ANSWER_PATH: &str = "answer";
pub fn answer_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    let answer_question = warp::path::end()
        .and(warp::post2())
        .and(sized_body_json(30))
        .and(optional_user_filter(state))
        .and(state.db())
        .map(answer_question_handler)
        .and_then(json_or_reject);

    // TODO need a get answers?
    // Put that under this subpath or questions?

    path(ANSWER_PATH).and(answer_question).boxed()
}

fn answer_question_handler(
    request: NewAnswerRequest,
    user_uuid: Option<Uuid>,
    conn: BoxedRepository,
) -> Result<Answer, Error> {
    let new_answer = NewAnswer {
        user_uuid,
        question_uuid: request.question_uuid,
        publicly_visible: request.publicly_visible,
        answer_text: request.answer_text,
    };
    conn.create_answer(new_answer).map_err(Error::from)
}
