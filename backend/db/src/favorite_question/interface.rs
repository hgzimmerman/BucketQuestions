//! Specification of what functions are supported for storing data for favorite questions.
use crate::{
    favorite_question::db_types::NewFavoriteQuestionRelation, question::db_types::Question,
};
use diesel::QueryResult;
use uuid::Uuid;

/// Functions for specifically working with Favorites.
pub trait FavoriteQuestionRelationRepository {
    /// Add the relation
    fn favorite_question(&self, relation: NewFavoriteQuestionRelation) -> QueryResult<()>;
    /// Removes the relation
    fn unfavorite_question(&self, relation: NewFavoriteQuestionRelation) -> QueryResult<()>;
    /// Gets the favorite quesitons.
    fn get_favorite_questions(&self, user_uuid: Uuid) -> QueryResult<Vec<Question>>;
}
