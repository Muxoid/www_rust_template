use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use sqlx;
use std::fmt;
use std::fmt::Debug;
use strum_macros::{Display, EnumIter};
use tracing_error::SpanTrace;

#[derive(Display, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, EnumIter, Hash)]
pub enum AppErrorType {
    IncorrectLogin,
    Display,
    InvalidInput,
    UserAlreadyExists,
    UserNotFound,
    ErrorDB,
    TemplateRenderError,
    Unknown(String), // A catch-all for errors not specifically categorized
}

pub struct AppError {
    pub error_type: AppErrorType,
    pub inner: Error,       // Underlying error information, if any
    pub context: SpanTrace, // Contextual tracing information
}

impl<T> From<T> for AppError
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        let cause = t.into();
        AppError {
            error_type: AppErrorType::Unknown(format!("{}", &cause)),
            inner: cause,
            context: SpanTrace::capture(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", &self.error_type)?;
        writeln!(f, "{}", self.inner)?;
        fmt::Display::fmt(&self.context, f)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppError")
            .field("message", &self.error_type)
            .field("inner", &self.inner)
            .field("context", &self.context)
            .finish()
    }
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self.error_type {
            AppErrorType::IncorrectLogin => {
                // Redirect to /login for IncorrectLogin error
                actix_web::HttpResponse::SeeOther()
                    .insert_header((actix_web::http::header::LOCATION, "/unauth"))
                    .finish()
            }
            // Handle other errors according to their types
            _ => {
                // Fallback to a default error response if the error type is not IncorrectLogin
                let status_code = self.status_code(); // Obtain the appropriate status code for the error
                actix_web::HttpResponse::build(status_code)
                    .content_type("application/json") // Example content type, adjust as needed
                    .body(format!(r#"{{"error": "{}"}}"#, self.to_string()))
            }
        }
    }
}

impl From<AppErrorType> for AppError {
    fn from(error_type: AppErrorType) -> Self {
        AppError {
            error_type,
            inner: anyhow!("User error occurred"), // Default message, can be customized
            context: SpanTrace::capture(),
        }
    }
}
