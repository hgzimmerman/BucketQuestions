use crate::{
    server_auth::create_google_oauth_client,
    state::{state_config::RunningEnvironment, State},
};
use authorization::Secret;
use db::{
    test::{
        fixture::Fixture,
        util::{execute_pool_test2, setup_fake_provider},
        TestType,
    },
    RepositoryProvider,
};
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use std::path::PathBuf;

impl State {
    /// Creates a new state object from an existing object pool.
    /// This is useful if using fixtures.
    #[cfg(test)]
    pub fn testing_init(repository_provider: RepositoryProvider, secret: Secret) -> Self {
        use std::time::Duration;
        let https = HttpsConnector::new(1).unwrap();
        let client = Client::builder()
            .keep_alive_timeout(Some(Duration::new(12, 0)))
            .build::<_, Body>(https);
        let redirect_url = RunningEnvironment::Staging { port: 8080 }.create_redirect_url();
        let google_oauth_client = create_google_oauth_client(redirect_url.clone());

        State {
            repository_provider,
            secret,
            https: client,
            google_oauth_client,
            server_lib_root: PathBuf::from("./"), // THIS makes the assumption that the tests are run from the backend/server dir.
            redirect_url,
        }
    }
}

/// This executes a test function on a repository that has been created in a manner
/// that it is guaranteed not to interfere with other tests.
#[cfg(test)]
pub fn execute_test_on_repository<Fix, Fun>(f: Fun)
where
    Fix: Fixture,
    Fun: Fn(&Fix, RepositoryProvider),
{
    match TestType::get_test_type_from_env() {
        TestType::Unit => {
            let (fixture, mock): (Fix, RepositoryProvider) = setup_fake_provider();
            f(&fixture, mock)
        }
        TestType::Integration => {
            execute_pool_test2(f);
        }
        TestType::Both => {
            let (fixture, mock): (Fix, RepositoryProvider) = setup_fake_provider();
            f(&fixture, mock);
            execute_pool_test2(f);
        }
    }
}
