use authorization::Secret;
use std::path::PathBuf;
use url::Url;
use crate::config::RepositoryType;
use crate::Config;

/// Configuration object for creating the state.
///
/// If unspecified, it will default to a sane default.
#[derive(Debug, Default)]
pub struct StateConfig {
    pub secret: Option<Secret>,
    pub max_pool_size: Option<u32>,
    pub server_lib_root: Option<PathBuf>,
    pub environment: RunningEnvironment,
    pub repository: RepositoryType
}

impl From<Config> for StateConfig {
    fn from(config: Config) -> Self {
        StateConfig {
            secret: config.secret,
            max_pool_size: config.max_pool_size,
            server_lib_root: config.server_lib_root,
            environment: config.running_environment,
            repository: config.repository
        }
    }
}


/// Where is the program running
#[derive(Debug, Clone)]
pub enum RunningEnvironment {
    /// Frontend is running off of `npm start`
    Node { port: u16 },
    /// Frontend is built, and served by the app, but accessible via 0.0.0.0:port
    Staging { port: u16 },
    /// Frontend is built and served by the app, and hidden behind a nginx reverse-proxy.
    /// This means that, the scheme may be https instead of http,
    /// and that the host will be an actual domain,
    /// and that it will implicitly be running on port 443.
    Production { origin: String },
}

impl Default for RunningEnvironment {
    fn default() -> Self {
        RunningEnvironment::Node { port: 3030 }
    }
}

impl RunningEnvironment {
    pub fn create_redirect_url(&self) -> Url {
        const PATH: &str = "api/auth/redirect";
        let url = match self {
            RunningEnvironment::Node { port } => format!("http://localhost:{}/{}", port, PATH),
            RunningEnvironment::Staging { port } => format!("http://localhost:{}/{}", port, PATH),
            RunningEnvironment::Production { origin } => format!("{}/{}", origin, PATH),
        };
        Url::parse(&url).expect("Could not parse url for redirect")
    }
}
