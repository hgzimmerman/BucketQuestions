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

pub mod answer;
pub mod bucket;
pub mod bucket_user_relation;
pub mod favorite_question;
pub mod mock;
pub mod question;
mod schema;
pub mod test;
pub mod user;
mod util;

use crate::{
    answer::interface::AnswerRepository, bucket::interface::BucketRepository,
    bucket_user_relation::interface::BucketUserRelationRepository,
    favorite_question::interface::FavoriteQuestionRelationRepository, mock::MockDatabase,
    question::interface::QuestionRepository, user::interface::UserRepository,
};
use diesel::PgConnection;
use pool::{Pool, PooledConn};
use std::{
    fmt::{Debug, Error, Formatter},
    sync::{Arc, Mutex},
};

// TODO Corsider replacing this with an enum, that has a method that does the same thing.
// This trait was useful in managaing a transition to an abstract Repository type, but now that that is done,
// a plain &PgConnection isn't useful, and the Repository trait could just be implemented for just PooledConn.
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
pub enum RepoAcquisitionError {
    /// The repository could not be gotten.
    CouldNotGetRepo,
}

/// Provides repositories
#[derive(Clone)]
pub enum RepositoryProvider {
    /// Pool repository provider
    Pool(Pool),
    /// Mock repository provider
    Mock(Arc<Mutex<MockDatabase>>),
}

impl Debug for RepositoryProvider {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            RepositoryProvider::Pool(_) => write!(f, "RepositoryProvider::Pool"),
            RepositoryProvider::Mock(mock) => mock.fmt(f),
        }
    }
}

/// An abstract repository that is sendable across threads
pub type BoxedRepository = Box<dyn Repository + Send>;

impl RepositoryProvider {
    /// Gets the repo.
    pub fn get_repo(&self) -> Result<BoxedRepository, RepoAcquisitionError> {
        match self {
            RepositoryProvider::Pool(pool) => {
                let repo = pool
                    .get()
                    .map_err(|_| RepoAcquisitionError::CouldNotGetRepo)?;
                Ok(Box::new(repo))
            }
            RepositoryProvider::Mock(mock) => {
                let repo = mock.clone();
                Ok(Box::new(repo))
            }
        }
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

#[cfg(test)]
mod unit {
    use super::*;
    use static_assertions;

    #[test]
    fn mock_is_repository() {
        static_assertions::assert_impl!(Arc<Mutex<MockDatabase>>, Repository)
    }

    #[test]
    fn pool_conn_is_repository() {
        static_assertions::assert_impl!(PooledConn, Repository)
    }
}
