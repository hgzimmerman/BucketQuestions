//! Responsible for serving static files and redirecting non-'/api/' requests to index.html.
use crate::api::API_STRING;
use apply::Apply;
use log::info;
use std::path::PathBuf;
use warp::{self, filters::BoxedFilter, fs::File, path::Peek, reply::Reply, Filter};

/// The directory that the webapp is stored in.
const ASSETS_DIRECTORY: &str = "../../frontend/build/"; // THIS ASSUMES THAT THE BINARY IS BUILT FROM THE ROOT DIRECTORY OF `backend/server`

/// Configuration object for setting up static files.
#[derive(Clone, Debug)]
pub struct FileConfig {
    static_dir_path: PathBuf,
    /// This is mostly to support testing.
    /// If set to Some, then the string in there will be used as the index,
    /// otherwise the app will assume that the static_dir_path/index.html is used for the index
    index_file_path: Option<PathBuf>,
}

impl FileConfig {
    pub fn new(root: PathBuf) -> Self {
        FileConfig {
            static_dir_path: root.join(ASSETS_DIRECTORY),
            index_file_path: None,
        }
    }

    fn index(&self) -> PathBuf {
        if let Some(index) = &self.index_file_path {
            index.clone()
        } else {
            self.static_dir_path.join("index.html")
        }
    }
}

/// Expose filters that work with static files
pub fn static_files_handler(file_config: FileConfig) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Static Files handler");

    let files = assets(file_config.static_dir_path.clone())
        .or(index_static_file_redirect(file_config.index()));

    warp::any()
        .and(files)
        .with(warp::log("static_files"))
        .boxed()
}

/// If the path does not start with /api, return the index.html, so the app will bootstrap itself
/// regardless of whatever the frontend-specific path is.
fn index_static_file_redirect(index_file_path: PathBuf) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path::peek())
        .and(warp::fs::file(index_file_path))
        .and_then(|segments: Peek, file: File| {
            // Reject the request if the path starts with /api/
            if let Some(first_segment) = segments.segments().next() {
                if first_segment == API_STRING {
                    return warp::reject::not_found().apply(Err);
                }
            }
            Ok(file)
        })
        .boxed()
}

/// Gets the file within the specified dir.
fn assets(dir_path: PathBuf) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::fs::dir(dir_path))
        .and(warp::path::end())
        .boxed()
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn index_test() {
        // request the main file from this crate.
        let x = warp::test::request()
            .path("/src/main.rs")
            .reply(&assets(PathBuf::from("./")));
        assert_eq!(x.status(), 200);
    }

    #[test]
    fn static_files_404() {
        let file_conf = FileConfig::new(PathBuf::from("./"));
        assert!(warp::test::request()
            .path("/api")
            .filter(&static_files_handler(file_conf))
            .is_err())
    }

    #[test]
    fn static_files_redirect_to_index() {
        let config = FileConfig {
            index_file_path: Some(PathBuf::from("./src/main.rs")),
            ..FileConfig::new(PathBuf::from("./"))
        };

        assert!(warp::test::request()
            .path("/yeet")
            .filter(&static_files_handler(config))
            .is_ok())
    }

    #[test]
    fn static_invalid_api_path_still_404s() {
        let file_conf = FileConfig::new(PathBuf::from("./"));
        let err = warp::test::request()
            .path("/api/yeet") // Matches nothing in the API space
            .filter(&static_files_handler(file_conf));

        let err: warp::Rejection = match err {
            Ok(_) => panic!("Error was expected, found valid Reply"),
            Err(e) => e,
        };
        assert!(err.is_not_found());
        //        let cause = err.find_cause::<Error>().ex();
        //        assert_eq!(*cause, Error::NotFound {type_name: "".to_string()})
    }
}
