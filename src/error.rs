// This format needs to be done by a macro so that we can generate "named" errors.

// Imports
use std::fmt::Display;
use serde::{Serialize, Serializer};
use warp::{http::StatusCode, reject::{Reject, Rejection}, reply, Reply};

/// Macro to generate the error types for the application.
macro_rules! error_types {
    (
        $($(#[$attr:meta])*  // Zero or more attributes user defined for the error type.
        $name:ident,         // The name of the error type.
    )*) => {
        $(
            // Apply the captured attributes to the generated struct.
            $(#[$attr])*
            #[derive(Debug)] // We derive Debug out of the box.
            pub struct $name;

            impl Reject for $name {}

            impl Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(stringify!($name))
                }
            }

            // Manually implement Serialize
            impl Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    // Helper function to convert PascalCase to UPPER_SNAKE_CASE
                    // WARNING: This is a very naive implementation. No consideration for Unicode or other edge cases.
                    fn to_upper_snake_case(s: &str) -> String {
                        let mut result = String::new();
                        for (i, c) in s.chars().enumerate() {
                            if c.is_uppercase() {
                                if i != 0 {
                                    result.push('_');
                                }
                                result.push(c);
                            } else {
                                result.push(c.to_ascii_uppercase());
                            }
                        }
                        result
                    }

                    let type_name = stringify!($name);
                    let converted_name = to_upper_snake_case(type_name);
                    serializer.serialize_str(&converted_name)
                }
            }

            impl std::error::Error for $name {}
        )*
    };
}

// Now we can generate the error types we need.
error_types! {
    /// Custom error type for unauthorized requests.
    Unauthorized,
    /// Custom error type for bad requests.
    BadRequest,
    /// Custom error type for not found requests.
    NotFound,
    /// Custom error type for internal server errors.
    InternalServerError,
}


/// Custom error type for sqlx errors.
#[derive(Debug)]
pub struct Sqlx(pub sqlx::Error);

/// Implement the [`warp::reject::Reject`] trait for [`Sqlx`].
impl warp::reject::Reject for Sqlx {}

/// Implement the [`std::fmt::Display`] trait for [`Sqlx`].
impl std::fmt::Display for Sqlx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// Implement the [`std::error::Error`] trait for [`Sqlx`].
impl std::error::Error for Sqlx {}

/// Custom rejection handler that maps rejections into responses.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    tracing::warn!("handle_rejection");
    tracing::warn!("{:?}", err);
    if err.is_not_found() {
        tracing::error_span!("not found");
        Ok(reply::with_status(warp::reply::json( &NotFound), StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<Sqlx>() {
        tracing::error!("sqlx error: {:?}", e);
        Ok(reply::with_status(warp::reply::json(&BadRequest), StatusCode::BAD_REQUEST))
    } else {
        eprintln!("unhandled rejection: {err:?}");
        Ok(reply::with_status(
        warp::reply::json(&InternalServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

