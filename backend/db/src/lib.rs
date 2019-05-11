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

pub mod bucket;
mod schema;
#[cfg(test)]
pub mod test;
pub mod user;
mod util;

use crate::{
    bucket::interface::{
        AnswerRepository, BucketRepository, BucketUserRelationRepository,
        FavoriteQuestionRelationRepository, QuestionRepository,
    },
    user::UserRepository,
};
use diesel::PgConnection;
use pool::{PooledConn, Pool};


/// Trait for anything that can resolve a reference to a Postgres Connection
pub trait AsConnRef {
    /// Get the postgres connection.
    fn as_conn(&self) -> &PgConnection;
}
impl AsConnRef for PooledConn {
    fn as_conn(&self) -> &PgConnection {
        &self
    }
}
impl AsConnRef for PgConnection {
    fn as_conn(&self) -> &PgConnection {
        self
    }
}

/// Errors that can occur when trying to get a repository.
#[derive(Clone, Copy, Debug)]
pub enum RepoAquisitionError {
    /// The repository could not be gotten.
    CouldNotGetRepo
}

/// Trait for anything that can resolve an implementor of a `Repository`.
pub trait RepoProvider {
    /// Gets the repo.
    fn get_repo(&self) -> Result<Box<Repository>, RepoAquisitionError>;
}
impl RepoProvider for Pool {
    fn get_repo(&self) -> Result<Box<Repository>, RepoAquisitionError> {
        let repo = self.get().map_err(|_| RepoAquisitionError::CouldNotGetRepo)?;
        let repo: Box<Repository> = Box::new(repo);
        Ok(repo)
    }
}



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
