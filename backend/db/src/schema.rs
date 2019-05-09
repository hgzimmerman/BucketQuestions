table! {
    answer (uuid) {
        uuid -> Uuid,
        user_uuid -> Nullable<Uuid>,
        question_uuid -> Uuid,
        publicly_visible -> Bool,
        answer_text -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    bq_user (uuid) {
        uuid -> Uuid,
        google_user_id -> Varchar,
        google_name -> Nullable<Varchar>,
    }
}

table! {
    bucket (uuid) {
        uuid -> Uuid,
        bucket_name -> Varchar,
        bucket_slug -> Varchar,
        public_viewable -> Bool,
        drawing_enabled -> Bool,
        exclusive -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    bucket_user_relation (user_uuid, bucket_uuid) {
        user_uuid -> Uuid,
        bucket_uuid -> Uuid,
        set_public_permission -> Bool,
        set_drawing_permission -> Bool,
        set_exclusive_permission -> Bool,
        grant_permissions_permission -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    question (uuid) {
        uuid -> Uuid,
        bucket_uuid -> Uuid,
        user_uuid -> Nullable<Uuid>,
        question_text -> Varchar,
        archived -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    user_question_favorite_relation (user_uuid, question_uuid) {
        user_uuid -> Uuid,
        question_uuid -> Uuid,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

joinable!(answer -> bq_user (user_uuid));
joinable!(answer -> question (question_uuid));
joinable!(bucket_user_relation -> bq_user (user_uuid));
joinable!(bucket_user_relation -> bucket (bucket_uuid));
joinable!(question -> bq_user (user_uuid));
joinable!(question -> bucket (bucket_uuid));
joinable!(user_question_favorite_relation -> bq_user (user_uuid));
joinable!(user_question_favorite_relation -> question (question_uuid));

allow_tables_to_appear_in_same_query!(
    answer,
    bq_user,
    bucket,
    bucket_user_relation,
    question,
    user_question_favorite_relation,
);
