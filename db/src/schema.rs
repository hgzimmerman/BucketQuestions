table! {
    answers (uuid) {
        uuid -> Uuid,
        user_uuid -> Nullable<Uuid>,
        question_uuid -> Uuid,
        publicly_visible -> Bool,
        answer_text -> Varchar,
    }
}

table! {
    buckets (uuid) {
        uuid -> Uuid,
        bucket_name -> Varchar,
        bucket_slug -> Varchar,
        visible -> Bool,
        drawing_enabled -> Bool,
    }
}

table! {
    bucket_user_join (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        bucket_uuid -> Uuid,
        set_visibility_permission -> Bool,
        set_drawing_permission -> Bool,
        grant_permissions_permission -> Bool,
    }
}

table! {
    questions (uuid) {
        uuid -> Uuid,
        bucket_uuid -> Uuid,
        user_uuid -> Nullable<Uuid>,
        question_text -> Varchar,
        archived -> Bool,
    }
}

table! {
    user_favorite_question_join (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        question_uuid -> Uuid,
    }
}

table! {
    users (uuid) {
        uuid -> Uuid,
        google_user_id -> Varchar,
        google_name -> Nullable<Varchar>,
    }
}

joinable!(answers -> questions (question_uuid));
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
