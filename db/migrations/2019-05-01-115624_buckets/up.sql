-- Your SQL goes here

CREATE TABLE buckets (
    uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    bucket_name VARCHAR NOT NULL,
    bucket_slug VARCHAR NOT NULL UNIQUE,
    visible BOOLEAN NOT NULL DEFAULT TRUE, -- Will anyone be able to see it in a list of visible buckets
    drawing_enabled BOOLEAN NOT NULL DEFAULT FALSE -- Is the bucket only accepting questions, or is there an active answer session going on.
);

CREATE TABLE bucket_user_join (
  uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
  bucket_uuid UUID NOT NULL REFERENCES buckets(uuid) ON DELETE CASCADE,
  set_visibility_permission BOOLEAN NOT NULL, -- Can the user make the bucket visible on the main page.
  set_drawing_permission BOOLEAN NOT NULL, -- Can the user set the mode to enable answering questions
  grant_permissions_permission BOOLEAN NOT NULL -- Can the user grant other users permissions for this bucket
);

CREATE TABLE questions (
  uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  bucket_uuid UUID NOT NULL REFERENCES buckets(uuid) ON DELETE CASCADE, -- The bucket the question belongs to.
  user_uuid UUID REFERENCES users(uuid) ON DELETE CASCADE, -- The user that created the Question. Users don't have to be logged in to create questions.
  question_text VARCHAR NOT NULL, -- The question
  archived BOOLEAN NOT NULL DEFAULT FALSE -- Once a question is answered, it is archived, and not able to be drawn again, unless explicitly put back in the bucket.
);

CREATE TABLE answers (
  uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  user_uuid UUID REFERENCES users(uuid) ON DELETE CASCADE, -- Users don't have to be logged in to answer the question
  question_uuid UUID NOT NULL REFERENCES questions(uuid) ON DELETE CASCADE,
  publicly_visible BOOLEAN NOT NULL DEFAULT FALSE, -- User's can look at their old answers, but may not want others to see them.
  answer_text VARCHAR NOT NULL -- The answer
);

-- Users can favorite questions for future reference.
-- This is only local to a given bucket.
-- The question will have to be copied if a favorited question is placed into another bucket
CREATE TABLE user_favorite_question_join (
  uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
  question_uuid UUID NOT NULL REFERENCES questions(uuid) ON DELETE CASCADE
);