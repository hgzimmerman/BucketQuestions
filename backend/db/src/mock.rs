//! Module for the database mock object.
use crate::user::{User, UserRepository, NewUser};
use crate::bucket::db_types::{Bucket, BucketUserJoin, Question, Answer, FavoriteQuestionRelation, NewBucket, BucketFlagChangeset, NewBucketUserJoin, BucketUserPermissionsChangeset, BucketUserPermissions, NewQuestion, NewAnswer, NewFavoriteQuestionRelation};
use diesel::result::{Error, DatabaseErrorKind, DatabaseErrorInformation};
use uuid::Uuid;
use std::sync::Mutex;
use crate::bucket::interface::{BucketRepository, BucketUserRelationRepository, QuestionRepository, AnswerRepository, FavoriteQuestionRelationRepository};
use rand::{Rng, thread_rng};

/// This isn't expected to match on the info provided by the actual database.
#[derive(Clone, Copy, Debug)]
pub struct DummyDbErrorInfo {
}
impl DummyDbErrorInfo {
    /// Creates a new DummyDbErrorInfo
    pub fn new() -> Self {
        DummyDbErrorInfo {}
    }
}

impl DatabaseErrorInformation for DummyDbErrorInfo {
    fn message(&self) -> &str {
        "Mock"
    }

    fn details(&self) -> Option<&str> {
        None
    }

    fn hint(&self) -> Option<&str> {
        None
    }

    fn table_name(&self) -> Option<&str> {
        None
    }

    fn column_name(&self) -> Option<&str> {
        None
    }

    fn constraint_name(&self) -> Option<&str> {
        None
    }
}

/// A mock object that should have parity with database operations.
#[derive(Debug, Clone, Default)]
pub struct MockDatabase {
    users: Vec<User>,
    buckets: Vec<Bucket>,
    user_bucket_relations: Vec<BucketUserJoin>,
    questions: Vec<Question>,
    answers: Vec<Answer>,
    favorite_question_relations: Vec<FavoriteQuestionRelation>
}


impl UserRepository for Mutex<MockDatabase> {
    fn create_user(&self, user: NewUser) -> Result<User, Error> {
        let uuid = Uuid::new_v4();
        let user = User {
            uuid,
            google_user_id: user.google_user_id,
            google_name: user.google_name
        };
        let mut db = self.lock().unwrap();
        if db.users.iter().find(|u| u.uuid == uuid).is_some() {
            return Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, Box::new(DummyDbErrorInfo::new())))
        }
        db.users.push(user.clone());
        return Ok(user)
    }

    fn get_user(&self, uuid: Uuid) -> Result<User, Error> {
        let db = self.lock().unwrap();
        db.users
            .iter()
            .find(|u| u.uuid == uuid)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }

    fn get_user_by_google_id(&self, id: String) -> Result<User, Error> {
        let db = self.lock().unwrap();
        db.users
            .iter()
            .find(|u| u.google_user_id == id)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }
}


impl BucketRepository for Mutex<MockDatabase> {
    fn create_bucket(&self, new_bucket: NewBucket) -> Result<Bucket, Error> {
        let mut db = self.lock().unwrap();
        let uuid = Uuid::new_v4();
        let bucket = Bucket {
            uuid,
            bucket_name: new_bucket.bucket_name,
            bucket_slug: new_bucket.bucket_slug,
            visible: true,
            drawing_enabled: true,
            private: false
        };
        if db.buckets.iter().find(|b| b.uuid == uuid).is_some() {
            return Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, Box::new(DummyDbErrorInfo::new())))
        }
        db.buckets.push(bucket.clone());
        return Ok(bucket)
    }

    fn delete_bucket(&self, bucket_uuid: Uuid) -> Result<Bucket, Error> {
        let mut db = self.lock().unwrap();
        let index = db.buckets.iter().position(|b| b.uuid == bucket_uuid).ok_or_else(|| Error::NotFound)?;
        Ok(db.buckets.remove(index))
    }

    fn get_publicly_visible_buckets(&self) -> Result<Vec<Bucket>, Error> {
        let db = self.lock().unwrap();
        let visible = db.buckets.iter().filter(|b| b.visible).cloned().collect();
        Ok(visible)
    }

    fn get_bucket_by_slug(&self, slug: String) -> Result<Bucket, Error> {
        let db = self.lock().unwrap();
        db.buckets.iter()
            .find(|b| b.bucket_slug == slug)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }

    fn get_bucket_by_uuid(&self, uuid: Uuid) -> Result<Bucket, Error> {
        let db = self.lock().unwrap();
        db.buckets.iter()
            .find(|b| b.uuid == uuid)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }

    fn change_bucket_flags(&self, changeset: BucketFlagChangeset) -> Result<Bucket, Error> {
        let mut db = self.lock().unwrap();
        let uuid = changeset.uuid;
        let bucket = db.buckets.iter_mut()
            .find(|b| b.uuid == uuid)
            .ok_or_else(|| Error::NotFound)?;

        if let Some(visible) = changeset.visible {
           bucket.visible = visible;
        }
        if let Some(drawing_enabled) = changeset.drawing_enabled {
            bucket.drawing_enabled = drawing_enabled;
        }
        if let Some(private) = changeset.private {
            bucket.private = private;
        }

        Ok(bucket.clone())
    }
}

impl BucketUserRelationRepository for Mutex<MockDatabase> {
    fn add_user_to_bucket(&self, relation: NewBucketUserJoin) -> Result<BucketUserJoin, Error> {
        let mut db = self.lock().unwrap();
        let relation = BucketUserJoin {
            user_uuid: relation.user_uuid,
            bucket_uuid: relation.bucket_uuid,
            set_visibility_permission: relation.set_visibility_permission,
            set_drawing_permission: relation.set_drawing_permission,
            set_private_permission: relation.set_private_permission,
            grant_permissions_permission: relation.grant_permissions_permission
        };
        if db.user_bucket_relations.iter().find(|r| {
            r.user_uuid == relation.user_uuid
                && r.bucket_uuid == relation.bucket_uuid
        }).is_some() {
            return Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, Box::new(DummyDbErrorInfo::new())))
        }
        db.user_bucket_relations.push(relation.clone());
        return Ok(relation)
    }

    fn remove_user_from_bucket(&self, user_uuid: Uuid, bucket_uuid: Uuid) -> Result<BucketUserJoin, Error> {
        let mut db = self.lock().unwrap();
        let index = db.user_bucket_relations.iter().position(|r| {
            r.user_uuid == user_uuid
                && r.bucket_uuid == bucket_uuid
        })
            .ok_or_else(|| Error::NotFound)?;

        Ok(db.user_bucket_relations.remove(index))
    }

    fn set_permissions(&self, permissions_changeset: BucketUserPermissionsChangeset) -> Result<BucketUserJoin, Error> {
        let mut db = self.lock().unwrap();
        let relation = db.user_bucket_relations.iter_mut()
            .find(|r| {
                r.user_uuid == permissions_changeset.user_uuid
                    && r.bucket_uuid == permissions_changeset.bucket_uuid
            })
            .ok_or_else(|| Error::NotFound)?;

        if let Some(visible) = permissions_changeset.set_visibility_permission {
           relation.set_visibility_permission = visible;
        }
        if let Some(drawing_enabled) = permissions_changeset.set_drawing_permission {
            relation.set_drawing_permission= drawing_enabled;
        }
        if let Some(private) = permissions_changeset.set_private_permission {
            relation.set_private_permission = private;
        }

        Ok(relation.clone())
    }

    fn get_permissions(&self, user_uuid: Uuid, bucket_uuid: Uuid) -> Result<BucketUserPermissions, Error> {
        let db = self.lock().unwrap();
        db.user_bucket_relations
            .iter()
            .find(|r| {
                r.user_uuid== user_uuid && r.bucket_uuid == bucket_uuid
            })
            .cloned()
            .ok_or_else(|| Error::NotFound)
            .map(|r| {
                BucketUserPermissions {
                    set_visibility_permission: r.set_visibility_permission,
                    set_drawing_permission: r.set_drawing_permission,
                    set_private_permission: r.set_private_permission,
                    grant_permissions_permission: r.grant_permissions_permission
                }
            })
    }

    fn get_buckets_user_is_a_part_of(&self, user_uuid: Uuid) -> Result<Vec<Bucket>, Error> {
        let db = self.lock().unwrap();
        let bucket_uuids: Vec<Uuid> = db.user_bucket_relations
            .iter()
            .filter(|r| r.user_uuid == user_uuid)
            .map(|r| r.bucket_uuid)
            .collect();
        let buckets = db.buckets
            .iter()
            .filter(|b| bucket_uuids.iter().any(|uuid| &b.uuid == uuid))
            .cloned()
            .collect();
        Ok(buckets)
    }

    fn get_users_in_bucket(&self, bucket_uuid: Uuid) -> Result<Vec<User>, Error> {
        let db = self.lock().unwrap();
        let user_uuids: Vec<Uuid> = db.user_bucket_relations
            .iter()
            .filter(|r| r.bucket_uuid == bucket_uuid)
            .map(|r| r.bucket_uuid)
            .collect();
        let users = db.users
            .iter()
            .filter(|b| user_uuids.iter().any(|uuid| &b.uuid == uuid))
            .cloned()
            .collect();
        Ok(users)
    }
}

impl QuestionRepository for Mutex<MockDatabase> {
    fn create_question(&self, question: NewQuestion) -> Result<Question, Error> {
        let uuid = Uuid::new_v4();
        let question = Question {
            uuid,
            bucket_uuid: question.bucket_uuid,
            user_uuid: question.user_uuid,
            question_text: question.question_text,
            archived: false
        };
        let mut db = self.lock().unwrap();
        if db.questions
            .iter()
            .find(|q| q.uuid == uuid).is_some() {
            return Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, Box::new(DummyDbErrorInfo::new())))
        }
        db.questions.push(question.clone());
        return Ok(question)
    }

    fn delete_question(&self, uuid: Uuid) -> Result<Question, Error> {
        let mut db = self.lock().unwrap();
        let index = db.questions.iter().position(|q| q.uuid == uuid).ok_or_else(|| Error::NotFound)?;
        Ok(db.questions.remove(index))
    }

    fn get_random_question(&self, bucket_uuid: Uuid) -> Result<Option<Question>, Error> {
        let db = self.lock().unwrap();
        let bucket_questions: Vec<&Question> = db.questions.iter().filter(|q| q.bucket_uuid == bucket_uuid).collect();
        let index: usize = thread_rng().gen_range(0, bucket_questions.len());
        Ok(bucket_questions.get(index).cloned().cloned())
    }

    fn get_number_of_active_questions_for_bucket(&self, bucket_uuid: Uuid) -> Result<i64, Error> {
        let db = self.lock().unwrap();
        let count = db.questions.iter().filter(|q| !q.archived && q.bucket_uuid == bucket_uuid ).count();
        Ok(count as i64)
    }

    fn get_all_questions_for_bucket_of_given_archived_status(&self, bucket_uuid: Uuid, archived: bool) -> Result<Vec<Question>, Error> {
        let db = self.lock().unwrap();
        let questions = db.questions
            .iter()
            .filter(|q| {
                q.archived == archived && q.bucket_uuid == bucket_uuid
            })
            .cloned()
            .collect();
        Ok(questions)
    }

    fn set_archive_status_for_question(&self, question_uuid: Uuid, archived: bool) -> Result<Question, Error> {
        let mut db = self.lock().unwrap();
        let question = db.questions.iter_mut().find(|q|q.uuid == question_uuid).ok_or_else(|| Error::NotFound)?;
        question.archived = archived;
        Ok(question.clone())
    }
}


impl AnswerRepository for Mutex<MockDatabase> {
    fn create_answer(&self, answer: NewAnswer) -> Result<Answer, Error> {
        let uuid = Uuid::new_v4();
        let answer = Answer {
            uuid,
            user_uuid: answer.user_uuid,
            question_uuid: answer.question_uuid,
            publicly_visible: answer.publicly_visible,
            answer_text: answer.answer_text
        };
        let mut db = self.lock().unwrap();
        if db.answers
            .iter()
            .find(|q| q.uuid == uuid).is_some() {
            return Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, Box::new(DummyDbErrorInfo::new())))
        }
        db.answers.push(answer.clone());
        return Ok(answer)
    }

    fn delete_answer(&self, uuid: Uuid) -> Result<Answer, Error> {
        let mut db = self.lock().unwrap();
        let index = db.answers.iter().position(|a| a.uuid == uuid).ok_or_else(|| Error::NotFound)?;
        Ok(db.answers.remove(index))
    }

    fn get_answers_for_question(&self, question_uuid: Uuid) -> Result<Vec<Answer>, Error> {
        let db = self.lock().unwrap();
        let answers = db.answers
            .iter()
            .filter(|a| a.question_uuid == question_uuid && a.publicly_visible)
            .cloned()
            .collect();
        Ok(answers)
    }
}

impl FavoriteQuestionRelationRepository for Mutex<MockDatabase> {
    fn favorite_question(&self, relation: NewFavoriteQuestionRelation) -> Result<(), Error> {
        let mut db = self.lock().unwrap();
        let relation = FavoriteQuestionRelation {
            user_uuid: relation.user_uuid,
            question_uuid: relation.question_uuid
        };
        if db.favorite_question_relations
            .iter()
            .find(|r| {
                r.user_uuid == relation.user_uuid
                && r.question_uuid == relation.question_uuid
        }).is_some() {
            return Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, Box::new(DummyDbErrorInfo::new())))
        }
        db.favorite_question_relations.push(relation.clone());
        return Ok(())
    }

    fn unfavorite_question(&self, relation: NewFavoriteQuestionRelation) -> Result<(), Error> {
        let mut db = self.lock().unwrap();
        let index = db.favorite_question_relations
            .iter()
            .position(|r| r.user_uuid == relation.user_uuid && r.question_uuid == relation.question_uuid)
            .ok_or_else(|| Error::NotFound)?;
        db.questions.remove(index);
        Ok(())
    }

    fn get_favorite_questions(&self, user_uuid: Uuid) -> Result<Vec<Question>, Error> {
        let db = self.lock().unwrap();
        let question_uuids: Vec<Uuid> = db.favorite_question_relations
            .iter()
            .filter(|f| f.user_uuid == user_uuid)
            .map(|f| f.question_uuid)
            .collect();
        let questions = db.questions
            .iter()
            .filter(|q| question_uuids.iter().any(|uuid| &q.uuid == uuid))
            .cloned()
            .collect();
        Ok(questions)
    }
}
