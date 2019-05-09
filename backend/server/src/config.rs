//! Configuration for the server.
use apply::Apply;
use clap::{App, Arg};

use crate::state::RunningEnvironment;
use authorization::Secret;
use log::{error, warn};
use std::path::PathBuf;

const DEFAULT_PORT: u16 = 8080;

/// Configuration options for initializing the server.
#[derive(Debug)]
pub struct Config {
    /// The port to start the server on.
    pub port: u16,
    /// If set to true, TLS will be enabled
    pub tls_enabled: bool,
    /// Command line defined secret. If none is provided, then the secret will be randomly generated.
    pub secret: Option<Secret>,
    /// The maximum size of the connection pool.
    /// If left unspecified, it will be left to the pool's discretion (At the time of writing, it defaults to 10)
    pub max_pool_size: Option<u32>,
    /// The root of the server lib.
    /// This is used to find static assets with and around the server crate.
    /// If the binary is launched from somewhere other than .../server, then this parameter needs to be supplied.
    pub server_lib_root: Option<PathBuf>,
    //    pub is_production: bool,
    /// What environment is the application running in?
    pub running_environment: RunningEnvironment,
}

impl Config {
    /// Parse the command line options and provide a configuration object.
    pub fn parse_command_line_arguments() -> Self {
        let matches = App::new("RIT SWEN 344 Server")
            .version("0.1.0")
            .author("Group 3")
            .about("Serves things")
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("PORT")
                    .help("The port to run the server on.")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("tls")
                    .long("tls")
                    .help("Run with TLS enabled. By default, TLS is not enabled.")
            )
            .arg(
                Arg::with_name("secret")
                    .long("secret")
                    .value_name("SECRET STRING")
                    .help("Initializes the secret to this value. It should be a long random string. If a secret is not provided, one will be randomly generated.")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("max_pool_size")
                    .long("max-pool-size")
                    .value_name("POOL SIZE")
                    .help("Number of connections the database pool supports. Defaults to 10.")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("server_lib_root")
                    .long("server-lib-root")
                    .value_name("PATH")
                    .help("The root of the server crate. Defaults to './'. Needs to be changed if the server is launched from somewhere other than '.../server'.")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("production")
                    .long("production")
                    .conflicts_with_all(&["development", "staging"])
                    .help("Run with configurations made for a production environment.")
            )
            .arg(
                Arg::with_name("development")
                    .long("development")
                    .conflicts_with_all(&["production", "staging"])
                    .help("Run with configurations made for a development environment, serving the frontend through npm start.")
            )
            .arg(
                Arg::with_name("staging")
                    .long("staging")
                    .conflicts_with_all(&["production", "development"])
                    .help("Run with configurations made for a staging environment.")
            )
            .get_matches_safe();

        match matches {
            Ok(matches) => {
                let port: u16 = if let Some(port) = matches.value_of("port") {
                    port.parse().expect("Port must be an integer")
                } else {
                    DEFAULT_PORT
                };

                let tls_enabled = matches.is_present("tls");

                let secret = matches.value_of("secret").map(Secret::new);

                let max_pool_size: u32 = if let Some(size) = matches.value_of("max_pool_size") {
                    size.parse().expect("Pool size must be an integer.")
                } else {
                    10 // There should be, by default, 10 database connections in the pool.
                };
                let max_pool_size = max_pool_size.apply(Some);

                let server_lib_root = matches.value_of("server_lib_root").map(PathBuf::from);

                let running_environment: RunningEnvironment = {
                    if matches.is_present("production") {
                        RunningEnvironment::Production {
                            origin: "https://weekendatjo.es".to_string(),
                        }
                    } else if matches.is_present("staging") {
                        RunningEnvironment::Staging { port }
                    } else if matches.is_present("development") {
                        RunningEnvironment::Node { port: 3000 }
                    } else {
                        warn!("Implicitly starting development environment in staging mode.");
                        RunningEnvironment::Staging { port }
                    }
                };

                Config {
                    port,
                    tls_enabled,
                    secret,
                    max_pool_size,
                    server_lib_root,
                    running_environment,
                }
            }
            Err(error) => {
                error!("Could not parse cli arguments: {}", error);
                panic!();
            }
        }
    }
}
