//! A library that contains all of the sql interfacing logic the server uses.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]

#[macro_use]
extern crate diesel;

//pub mod adaptive_health;
//pub mod event;
mod schema;
//pub mod stock;
pub mod bucket;
pub mod user;
pub mod mock;
mod util;
#[cfg(test)]
mod test;

use crate::{
    bucket::interface::{
        AnswerRepository, BucketRepository, BucketUserRelationRepository,
        FavoriteQuestionRelationRepository, QuestionRepository,
    },
    user::UserRepository,
};

/// A trait that encompasses all repository traits.
///
/// Putting the database methods behind a trait
/// allows for the injection of mock database objects instead,
/// which allows unit testing of business logic.
pub trait Repository:
    BucketRepository
    + BucketUserRelationRepository
    + QuestionRepository
    + AnswerRepository
    + FavoriteQuestionRelationRepository
    + UserRepository
{
}

// Blanket impl
impl<T> Repository for T where
    T: BucketRepository
        + BucketUserRelationRepository
        + QuestionRepository
        + AnswerRepository
        + FavoriteQuestionRelationRepository
        + UserRepository
{
}
