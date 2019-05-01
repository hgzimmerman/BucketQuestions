table! {
    answers (uuid) {
        uuid -> Uuid,
        user_uuid -> Nullable<Uuid>,
        publicly_visible -> Bool,
        answer_text -> Varchar,
    }
}

table! {
    buckets (uuid) {
        uuid -> Uuid,
        bucket_name -> Varchar,
        visible -> Bool,
        accepting_answers -> Bool,
    }
}

table! {
    bucket_user_join (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        bucket_uuid -> Uuid,
        set_visibility_permission -> Bool,
        set_accepting_answers_permission -> Bool,
        grant_permissions_permission -> Bool,
    }
}

table! {
    questions (uuid) {
        uuid -> Uuid,
        bucket_uuid -> Nullable<Uuid>,
        user_uuid -> Nullable<Uuid>,
        question_text -> Varchar,
        archived -> Bool,
    }
}

table! {
    user_favorite_question_join (uuid) {
        uuid -> Uuid,
        user_uuid -> Nullable<Uuid>,
        question_uuid -> Nullable<Uuid>,
    }
}

table! {
    users (uuid) {
        uuid -> Uuid,
        google_user_id -> Int4,
    }
}

joinable!(answers -> users (user_uuid));
joinable!(bucket_user_join -> buckets (bucket_uuid));
joinable!(bucket_user_join -> users (user_uuid));
joinable!(questions -> buckets (bucket_uuid));
joinable!(questions -> users (user_uuid));
joinable!(user_favorite_question_join -> questions (question_uuid));
joinable!(user_favorite_question_join -> users (user_uuid));

allow_tables_to_appear_in_same_query!(
    answers,
    buckets,
    bucket_user_join,
    questions,
    user_favorite_question_join,
    users,
);
