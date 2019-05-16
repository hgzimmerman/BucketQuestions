//! This is a crate for wrapping common JWT functionality needed for securing information in a webapp.
//! It is flexible in that it can support arbitrary payload subjects.
//!
//! It currently only supports HS256 keys.
//!

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

use chrono::{Duration, NaiveDateTime};
use frank_jwt::{decode, encode, Algorithm};
use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{self, Debug, Display, Error, Formatter};

/// Enumeration of all errors that can occur while authenticating.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum AuthError {
    /// Used to indicate that the signature does not match the hashed contents + secret
    IllegalToken,
    /// The expired field in the token is in the past
    ExpiredToken,
    /// The request did not have a token.
    MissingToken,
    /// The JWT 'bearer schema' was not followed.
    MalformedToken,
    /// Could not deserialize the base64 encoded JWT.
    DeserializeError,
    /// Could not serialize the JWT to base64.
    SerializeError,
    /// Could not decode the JWT.
    JwtDecodeError,
    /// Could not encode the JWT.
    JwtEncodeError,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let description = match self {
            AuthError::DeserializeError => "JWT could not be deserialized.",
            AuthError::SerializeError => "JWT could not be serialized.",
            AuthError::JwtDecodeError => "JWT could not be decoded.",
            AuthError::JwtEncodeError => "JWT could not be encoded.",
            AuthError::IllegalToken => "The provided token is invalid.",
            AuthError::ExpiredToken => {
                "The provided token has expired, please reauthenticate to acquire a new one."
            }
            AuthError::MalformedToken => "The token was not formatted correctly.",
            AuthError::MissingToken => {
                "A JWT token was expected and none was provided. Try logging in."
            }
        };

        write!(f, "{}", description)
    }
}

/// The payload section of the JWT
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JwtPayload<T> {
    /// Issue date of the token
    pub iat: NaiveDateTime,
    /// Subject - the information being authenticated by this token
    pub sub: T,
    /// Expiration date of the token
    pub exp: NaiveDateTime,
}

impl<T> JwtPayload<T>
where
    for<'de> T: Serialize + Deserialize<'de> + Send,
{
    /// Creates a new token for the subject that will expire after a specified time.
    ///
    /// # Arguments
    /// * subject - The subject of the JWT, it holds the contents that can be trusted by the server on return trips.
    /// * lifetime - How long the JWT will be valid for after its creation.
    ///
    /// # Example
    /// ```
    /// # use authorization::JwtPayload;
    /// let payload = JwtPayload::new("hello".to_string(), chrono::Duration::weeks(2));
    /// ```
    pub fn new(subject: T, lifetime: Duration) -> Self {
        let now = chrono::Utc::now().naive_utc();

        JwtPayload {
            iat: now,
            sub: subject,
            exp: now + lifetime,
        }
    }

    /// Gets the subject of the JWT payload.
    ///
    /// # Example
    /// ```
    /// # use authorization::{JwtPayload};
    /// let payload = JwtPayload::new("hello".to_string(), chrono::Duration::weeks(4));
    /// let subject = payload.subject();
    /// assert_eq!(subject, "hello".to_string());
    /// ```
    pub fn subject(self) -> T {
        self.sub
    }

    /// Validates if the token is expired or not.
    /// It also checks if the token was issued in the future, to further complicate the attack
    /// surface of someone creating forgeries.
    ///
    /// # Example
    /// ```
    /// # use authorization::{AuthError, JwtPayload};
    /// # fn main() -> Result<(), AuthError> {
    /// let payload = JwtPayload::new("hello".to_string(), chrono::Duration::weeks(4));
    /// let payload = payload.validate_dates()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn validate_dates(self) -> Result<Self, AuthError> {
        let now = chrono::Utc::now().naive_utc();
        if self.exp < now || self.iat > now {
            Err(AuthError::ExpiredToken)
        } else {
            Ok(self)
        }
    }

    /// Encodes the payload, producing a JWT String.
    ///
    /// # Example
    /// ```
    /// # use authorization::AuthError;
    /// # fn main() -> Result<(), AuthError> {
    /// # use authorization::{JwtPayload, Secret};
    /// let payload = JwtPayload::new("hello".to_string(), chrono::Duration::weeks(2));
    /// let secret = Secret::new_hmac("Secret".to_string());
    /// let jwt = payload.encode_jwt_string(&secret)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn encode_jwt_string(&self, secret: &Secret) -> Result<String, AuthError> {
        let header = json!({});
        use serde_json::Value;

        let payload: Value = match serde_json::to_value(&self) {
            Ok(x) => x,
            Err(_) => return Err(AuthError::SerializeError),
        };

        match secret {
            Secret::Hmac(key) => {
                encode(header, key, &payload, Algorithm::HS256)
            }
            Secret::Rsa { private_key, .. } => {
                encode(header, private_key, &payload, Algorithm::RS512)
            }
            Secret::Es { private_key, .. } => {
                encode(header, private_key, &payload, Algorithm::ES512)
            }
        }
            .map_err(|_| AuthError::JwtEncodeError)


    }

    /// Decodes the JWT into its payload.
    /// If the signature doesn't match, then a decode error is thrown.
    ///
    /// # Example
    /// ```
    /// # use authorization::AuthError;
    /// # fn main() -> Result<(), AuthError> {
    /// # use authorization::{JwtPayload, Secret};
    /// # let secret = Secret::new_hmac("Secret".to_string());
    /// let payload = JwtPayload::new("hello".to_string(), chrono::Duration::weeks(2));
    /// let jwt: String = payload.encode_jwt_string(&secret)?;
    /// let decoded_payload: JwtPayload<String> = JwtPayload::decode_jwt_string(&jwt, &secret)?;
    /// assert_eq!(payload, decoded_payload);
    /// # Ok(())
    /// # }
    /// ```
    pub fn decode_jwt_string(jwt_str: &str, secret: &Secret) -> Result<JwtPayload<T>, AuthError> {
        let (_header, payload) = match secret {
            Secret::Hmac(key) => {
                decode(&jwt_str.to_string(), key, Algorithm::HS256)
            }
            Secret::Rsa { public_key, .. } => {
                decode(&jwt_str.to_string(), public_key, Algorithm::RS512)
            }
            Secret::Es { public_key, .. } => {
                decode(&jwt_str.to_string(), public_key, Algorithm::ES512)
            }
        }
            .map_err(|_| AuthError::JwtDecodeError)?;

        let jwt: JwtPayload<T> = serde_json::from_value(payload).map_err(|_| AuthError::DeserializeError)?;
        Ok(jwt)
    }

    /// Extracts the JWT from the bearer string, and decodes it to determine if it was signed properly.
    ///
    /// # Example
    /// ```
    /// # use authorization::AuthError;
    /// # fn main() -> Result<(), AuthError> {
    /// # use authorization::{JwtPayload, Secret, AuthError};
    /// # let payload = JwtPayload::new("hello".to_string(), chrono::Duration::weeks(2));
    /// # let secret = Secret::new_hmac("Secret".to_string());
    /// let jwt: String = payload.encode_jwt_string(&secret)?;
    /// let bearer_string = format!("bearer {}", jwt);
    /// let decoded_payload: JwtPayload<String> = JwtPayload::extract_jwt(bearer_string, &secret)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn extract_jwt(bearer_string: String, secret: &Secret) -> Result<JwtPayload<T>, AuthError> {
        let authorization_words: Vec<String> =
            bearer_string.split_whitespace().map(String::from).collect();

        if authorization_words.len() != 2 {
            return Err(AuthError::MissingToken);
        }
        if authorization_words[0] != BEARER {
            return Err(AuthError::MalformedToken);
        }
        let jwt_str: &str = &authorization_words[1];

        JwtPayload::decode_jwt_string(jwt_str, secret).map_err(|_| AuthError::IllegalToken)
    }
}

/// Secret used for authentication
///
///
/// # Warning
/// No guarantees are made about Rsa and Es variants working yet.
#[derive(Clone)]
pub enum Secret {
    /// HMAC secret
    Hmac(String),
    /// RSA secrets
    Rsa {
        /// Private key
        private_key: String,
        /// Public key
        public_key: String
    },
    /// Es secrets
    Es {
        /// Private key
        private_key: String,
        /// Public key
        public_key: String
    }
}
impl Debug for Secret {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (first_five_letters, length) = match self {
            Secret::Hmac(key) => {
                (key.chars().take(5).collect::<String>(), key.len())
            },
            Secret::Rsa {private_key, ..} => {
                (private_key.chars().take(5).collect::<String>(), private_key.len())
            },
            Secret::Es {private_key, ..} => {
                (private_key.chars().take(5).collect::<String>(), private_key.len())
            },
        };
        f.debug_struct("Secret")
            .field("secret", &format!("{}[REDACTED]", first_five_letters))
            .field("(secret_length)", &format!("{}", length))
            .finish()
    }
}

impl Secret {
    /// Create a new HMAC secret.
    pub fn new_hmac(key: String) -> Self {
        if key.len() < 100 {
            warn!("HMAC key is fewer than 100 characters");
        }
        Secret::Hmac(key)
    }

    /// Create a new RSA secret
    pub fn new_rsa(private_key: String, public_key: String) -> Self {
        Secret::Rsa{private_key, public_key}
    }
    /// Create a new Es secret
    pub fn new_es(private_key: String, public_key: String) -> Self {
        Secret::Es{private_key, public_key}
    }

}

/// The prefix before the encoded JWT in the header value that corresponds to the "Authorization" key.
pub const BEARER: &str = "bearer";
/// The key used in the header to map to the authentication data.
pub const AUTHORIZATION_HEADER_KEY: &str = "Authorization";

#[cfg(test)]
mod test {
    use super::*;

    /// Tests if a jwt payload can be encoded and then decoded.
    #[test]
    fn encode_decode() {
        let payload = JwtPayload::new("hello_there".to_string(), Duration::weeks(2));
        let secret = Secret::new_hmac("secret".to_string());

        let encoded = payload.encode_jwt_string(&secret).unwrap();
        let decoded = JwtPayload::<String>::decode_jwt_string(&encoded, &secret).unwrap();

        assert_eq!(decoded, payload)
    }

    /// Tests if a jwt can be extracted from a bearer string.
    #[test]
    fn encode_extract() {
        let payload = JwtPayload::new("hello_there".to_string(), Duration::weeks(2));
        let secret = Secret::new_hmac("secret".to_string());
        let encoded = payload.encode_jwt_string(&secret).unwrap();
        let header_string = format!("{} {}", BEARER, encoded);

        let decoded = JwtPayload::<String>::extract_jwt(header_string, &secret).unwrap();
        assert_eq!(decoded, payload)
    }

}
