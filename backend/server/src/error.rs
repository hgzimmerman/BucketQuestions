//! Responsible for enumerating all the possible ways the server may encounter undesired states.
//!
//! It handles serializing these errors so that they can be consumed by the user of the api.
//!
//! Currently, this is tightly coupled to both Warp, Diesel, and Authorization.
//! It likely should be broken off into its own crate with the warp-related functions allowed as a feature.
use apply::Apply;
use authorization::AuthError;
use diesel::result::DatabaseErrorKind;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    error::Error as StdError,
    fmt::{self, Display},
};
use strum_macros::AsRefStr;
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

/// Server-wide error variants.
/// These integrate tightly with the error rewriting infrastructure provided by `warp`.
///
/// They should be mostly concerned with business-logic errors or errors regarding
/// user-implementation problems with warp (eg, pools being exhausted).
#[derive(Debug, Clone, PartialEq, Serialize, AsRefStr)]
pub enum Error {
    /// The provider for connections could not provide a connection to the database.
    DatabaseUnavailable,
    /// The database encountered an error while running a query.
    DatabaseError(String),
    /// The client expected the server to be in a given state, but it was not, so the request was rejected.
    PreconditionNotMet(String),
    /// If the server needs to talk to an external API to properly serve a request,
    /// and that server experiences an error, this is the error to represent that.
    DependentConnectionFailed(DependentConnectionError),
    /// The server encountered an unspecified error.
    InternalServerError(Option<String>),
    /// The requested entity could not be located.
    NotFound { type_name: String },
    /// The request was bad, with a dynamic reason.
    BadRequest(String),
    /// The user does not have access to a particular resource.
    /// Authorization - user may be authenticated, but still should not access the resource.
    /// This is synonymous with HTTP - Forbidden code.
    NotAuthorized { reason: String },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DependentConnectionError {
    Url(String),
    Context(String),
    UrlAndContext(String, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description: String = match self {
            Error::DatabaseUnavailable => {
                "Could not acquire a connection to the database, the connection pool may be overloaded with requests.".to_string()
            }
            Error::DatabaseError(e) => e.to_string(),
            Error::PreconditionNotMet(e) => e.to_string(),
            Error::BadRequest(s)=> s.to_string(),
            Error::InternalServerError(s) => {
                if let Some(e) = s {
                    e.clone()
                } else {
                    "Internal server error encountered.".to_string()
                }
            },
            Error::DependentConnectionFailed(error) => {
                match error {
                    DependentConnectionError::Context(reason) => {
                        format!("An internal request needed to serve the request failed. With reason: '{}'", reason)
                    },
                    DependentConnectionError::Url(uri) => {
                        format!("An internal request needed to serve the request failed. Dependent url: '{}'", uri)
                    },
                    DependentConnectionError::UrlAndContext(uri, reason) => {
                        format!("An internal request needed to serve the request failed. Dependent url: {}. With reason: '{}'", uri, reason)
                    }
                }
            },
            Error::NotFound { type_name } => {
                format!("The resource ({}) you requested could not be found.", type_name)
            }
            Error::NotAuthorized { reason } => {
                format!("You are forbidden from accessing this resource. ({})", reason)
            }
        };
        write!(f, "{}", description)
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl Error {
    fn as_error_response(&self) -> ErrorResponse {
        info!("ERROR: {:?} | message: {}", self, self);
        let message = self.to_string();

        let code: StatusCode = self.error_code();
        let tag: &AsRef<str> = &self;
        let tag: String = tag.as_ref().to_string();

        let error_response = ErrorResponse {
            tag,
            message,
            canonical_reason: code.canonical_reason().unwrap_or_default().to_string(),
            error_code: code.as_u16(),
        };
        error_response
    }
}

/// Converts the auth error into an ErrorResponse
fn auth_error_as_error_response(auth_error: &AuthError) -> ErrorResponse {
    info!("ERROR: {:?} | message: {}", auth_error, auth_error);

    let message = auth_error.to_string();
    let code: StatusCode = auth_error_code(&auth_error);
    let tag: &AsRef<str> = &auth_error;
    let tag: String = tag.as_ref().to_string();

    let error_response = ErrorResponse {
        tag,
        message,
        canonical_reason: code.canonical_reason().unwrap_or_default().to_string(),
        error_code: code.as_u16(),
    };
    error_response
}

fn auth_error_code(auth_error: &AuthError) -> StatusCode {
    match auth_error {
        AuthError::IllegalToken => StatusCode::UNAUTHORIZED,
        AuthError::ExpiredToken => StatusCode::UNAUTHORIZED,
        AuthError::MalformedToken => StatusCode::UNAUTHORIZED, // Unauthorized is for requests that require authentication and the authentication is out of date or not present
        AuthError::MissingToken => StatusCode::UNAUTHORIZED,
        AuthError::DeserializeError => StatusCode::INTERNAL_SERVER_ERROR,
        AuthError::SerializeError => StatusCode::INTERNAL_SERVER_ERROR,
        AuthError::JwtDecodeError => StatusCode::UNAUTHORIZED,
        AuthError::JwtEncodeError => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Takes a rejection, which Warp would otherwise handle in its own way, and transform it into
/// an `Ok(Reply)` where the status is set to correspond to the provided error.
///
/// # Note
/// This only works if the Rejection is of the custom Error type. Any others will just fall through this unchanged.
///
/// This should be used at the top level of the exposed api.
///
/// # Arguments
/// * err - A `Rejection` that will be rewritten into an `ErrorResponse`.
///
pub fn customize_error(err: Rejection) -> Result<impl Reply, Rejection> {

    let error_response = err.find_cause::<Error>()
        .map(|cause: &Error| {
            cause.as_error_response()
        })
        .or_else(|| {
            err.find_cause::<AuthError>()
                .map(|cause: &AuthError| {
                    auth_error_as_error_response(&cause)
                })
        })
        .or_else(|| {
            err.find_cause::<diesel::result::Error>()
                .map( |_cause: &diesel::result::Error| {
                    let code = StatusCode::INTERNAL_SERVER_ERROR;
                    ErrorResponse {
                        tag: "DatabaseError".to_string(),
                        message: "Some aspect of database interaction failed".to_string(), // TODO make this specific
                        canonical_reason: code.canonical_reason().unwrap_or_default().to_string(),
                        error_code: code.as_u16(),
                    }
                })
        })
        .or_else(|| {
            // Fall back to just matching on the status given by the default warp impl.
            match err.status() {
                c @ StatusCode::METHOD_NOT_ALLOWED => {
                    Some(
                        ErrorResponse {
                            tag: "MethodNotAllowed".to_string(),
                            message: "Http method not allowed".to_string(),
                            canonical_reason: c.canonical_reason().unwrap_or_default().to_string(),
                            error_code: c.as_u16(),
                        }
                    )
                }
                StatusCode::NOT_FOUND => {
                    Some(Error::NotFound {
                        type_name: "Resource not found".to_string(),
                    }.as_error_response()
                    )
                }
                _ => None
            }
        })
        .unwrap_or_else(|| {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            error!("UNHANDLED ERROR: {:#?}", err);
            ErrorResponse {
                tag: "Unhandled".to_string(),
                message: "Unhandled error".to_string(),
                canonical_reason: status_code.canonical_reason().unwrap_or_default().to_string(),
                error_code: status_code.as_u16(),
            }
        });

    // Expecting is safe here because all error responses came from status codes in the first place.
    let code = StatusCode::from_u16(error_response.error_code).expect("Error code did not come from a status code.");
    let json = warp::reply::json(&error_response);
    Ok(warp::reply::with_status(json, code))
}

impl Error {
    /// Get the error code correlated with the status code.
    fn error_code(&self) -> StatusCode {
        match *self {
            Error::DatabaseUnavailable => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::PreconditionNotMet(_) => StatusCode::PRECONDITION_FAILED,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound { .. } => StatusCode::NOT_FOUND,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DependentConnectionFailed(_) => StatusCode::BAD_GATEWAY,
            Error::NotAuthorized { .. } => StatusCode::FORBIDDEN,
        }
    }

    /// Reject an error into a Result `Err`.
    /// This is useful under some circumstances when returning a specific error from within Warp.
    pub fn reject_result<T>(self) -> Result<T, Rejection> {
        Err(warp::reject::custom(self))
    }

    /// Reject an error.
    #[inline]
    pub fn reject(self) -> Rejection {
        warp::reject::custom(self)
    }

    /// Transform a compatible error and reject it.
    #[inline]
    #[allow(dead_code)]
    pub fn from_reject<T: Into<Error>>(error: T) -> Rejection {
        error.into().apply(Self::reject)
    }

    /// Construct a bad request error.
    #[inline]
    #[allow(dead_code)]
    pub fn bad_request<T: Into<String>>(message: T) -> Self {
        Error::BadRequest(message.into())
    }
    /// Construct an internal error with a custom message.
    #[inline]
    pub fn internal_server_error<T: Into<String>>(reason: T) -> Self {
        Error::InternalServerError(Some(reason.into()))
    }

    /// Construct a generic internal error.
    #[allow(dead_code)]
    pub fn internal_server_error_empty() -> Self {
        Error::InternalServerError(None)
    }

    /// Construct a gateway error that includes the context about why the connection failed.
    #[inline]
    #[allow(dead_code)]
    pub fn dependent_connection_failed_context<T: Into<String>>(context: T) -> Self {
        Error::DependentConnectionFailed(DependentConnectionError::Context(context.into()))
    }

    /// Construct a gateway error that includes the url.
    #[inline]
    #[allow(dead_code)]
    pub fn dependent_connection_failed_url<T: Into<String>>(url: T) -> Self {
        Error::DependentConnectionFailed(DependentConnectionError::Url(url.into()))
    }

    /// Construct a gateway error that includes the url and a contextual note about the request.
    #[inline]
    #[allow(dead_code)]
    pub fn dependent_connection_failed<T: Into<String>, U: Into<String>>(
        url: T,
        reason: U,
    ) -> Self {
        Error::DependentConnectionFailed(DependentConnectionError::UrlAndContext(
            url.into(),
            reason.into(),
        ))
    }

    /// Construct a not found error with the name of the type that could not be found.
    #[allow(dead_code)]
    #[inline]
    pub fn not_found<T: Into<String>>(type_name: T) -> Self {
        Error::NotFound {
            type_name: type_name.into(),
        }
    }

    /// Construct a not authorized error with a reason.
    #[allow(dead_code)]
    #[inline]
    pub fn not_authorized<T: ToString>(reason: T) -> Self {
        Error::NotAuthorized {
            reason: reason.to_string(),
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        use self::Error::*;
        use diesel::result::Error as DieselError;
        match error {
            DieselError::DatabaseError(e, _) => {
                let e = match e {
                    DatabaseErrorKind::ForeignKeyViolation => {
                        "A foreign key constraint was violated in the database."
                    }
                    DatabaseErrorKind::SerializationFailure => {
                        "Value failed to serialize in the database."
                    }
                    DatabaseErrorKind::UnableToSendCommand => {
                        "Database Protocol violation, possibly too many bound parameters."
                    }
                    DatabaseErrorKind::UniqueViolation => {
                        "A unique constraint was violated in the database"
                    }
                    DatabaseErrorKind::__Unknown => "An unknown error occurred in the database.",
                }
                .to_string();
                DatabaseError(e)
            }
            DieselError::NotFound => NotFound {
                type_name: "Database Entity".to_string(),
            },
            e => {
                error!("Unhandled database error: '{}'", e);
                InternalServerError(None)
            }
        }
    }
}

/// Error response template for when the errors are rewritten.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub tag: String,
    pub message: String,
    pub canonical_reason: String,
    pub error_code: u16,
}
