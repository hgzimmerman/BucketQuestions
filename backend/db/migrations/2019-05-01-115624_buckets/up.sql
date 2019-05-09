-- Your SQL goes here

CREATE TABLE bucket (
    uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    bucket_name VARCHAR NOT NULL,
    bucket_slug VARCHAR NOT NULL UNIQUE,
    public_viewable BOOLEAN NOT NULL DEFAULT TRUE, -- Will anyone be able to see it in a list of visible buckets
    drawing_enabled BOOLEAN NOT NULL DEFAULT TRUE, -- Is the bucket only accepting questions, or is there an active answer session going on.
    exclusive BOOLEAN NOT NULL DEFAULT FALSE, -- Can the bucket be joined if there isn't a relation in the bucket_user_join table?
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE bucket_user_relation (
  user_uuid UUID NOT NULL REFERENCES bq_user(uuid) ON DELETE CASCADE,
  bucket_uuid UUID NOT NULL REFERENCES bucket(uuid) ON DELETE CASCADE,
  set_public_permission BOOLEAN NOT NULL, -- Can the user make the bucket visible on the main page.
  set_drawing_permission BOOLEAN NOT NULL, -- Can the user set the mode to enable answering questions.
  set_exclusive_permission BOOLEAN NOT NULL, -- Can the user set the mode to disable joining by random individuals.
  grant_permissions_permission BOOLEAN NOT NULL, -- Can the user grant other users permissions for this bucket.
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_uuid, bucket_uuid)
);

CREATE TABLE question (
  uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  bucket_uuid UUID NOT NULL REFERENCES bucket(uuid) ON DELETE CASCADE, -- The bucket the question belongs to.
  user_uuid UUID REFERENCES bq_user(uuid) ON DELETE CASCADE, -- The user that created the Question. Users don't have to be logged in to create questions.
  question_text VARCHAR NOT NULL, -- The question
  archived BOOLEAN NOT NULL DEFAULT FALSE, -- Once a question is answered, it is archived, and not able to be drawn again, unless explicitly put back in the bucket.
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE answer (
  uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  user_uuid UUID REFERENCES bq_user(uuid) ON DELETE CASCADE, -- Users don't have to be logged in to answer the question
  question_uuid UUID NOT NULL REFERENCES question(uuid) ON DELETE CASCADE,
  publicly_visible BOOLEAN NOT NULL DEFAULT FALSE, -- User's can look at their old answers, but may not want others to see them.
  answer_text VARCHAR NOT NULL, -- The answer
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Users can favorite questions for future reference.
-- This is only local to a given bucket.
-- The question will have to be copied if a favorited question is placed into another bucket
CREATE TABLE user_question_favorite_relation (
  user_uuid UUID NOT NULL REFERENCES bq_user(uuid) ON DELETE CASCADE,
  question_uuid UUID NOT NULL REFERENCES question(uuid) ON DELETE CASCADE,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_uuid, question_uuid)
);