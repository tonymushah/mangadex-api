use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ResultType;

use crate::RelationshipType;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum RelationshipConversionError {
    #[error("The input relationship type {input} is incompatible with {inner}")]
    InvalidInputRelationshipType {
        input: RelationshipType,
        inner: RelationshipType,
    },
    #[error("The {0} related attributes is not found")]
    AttributesNotFound(RelationshipType),
}

#[derive(Debug, thiserror::Error, Deserialize, Serialize)]
#[error("Bad request")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct MangaDexErrorResponse_ {
    #[serde(default = "ResultType::error")]
    pub result: ResultType,
    #[serde(default)]
    pub errors: Vec<MangaDexError>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Deserialize, Clone, Serialize, Default)]
#[error("API error")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct MangaDexError {
    pub id: Uuid,
    /// HTTP status code.
    pub status: u16,
    /// Error title.
    pub title: Option<String>,
    /// Description about the error.
    pub detail: Option<String>,
    /// Provides insight into why the request failed.
    ///
    /// # Captcha Errors (400)
    ///
    /// The error may have been caused by one of the following:
    ///
    /// - Captcha challenge result was wrong.
    /// - The Captcha Verification service was down.
    /// - Other, refer to the error message and the `errorCode` value.
    ///
    /// # Rate Limit, Captcha Required (403)
    ///
    /// Some endpoints may require captchas to proceed, in order to slow down automated malicious
    /// traffic. Legitimate users might also be affected, based on the frequency of write requests
    /// or due certain endpoints being particularly sensitive to malicious use, such as user signup.
    ///
    /// Once an endpoint decides that a captcha needs to be solved,
    /// a 403 Forbidden response will be returned, with the error code `captcha_required_exception`.
    /// The sitekey needed for recaptcha to function is provided in both the
    /// `X-Captcha-Sitekey` header field, as well as in the error context,
    /// specified as `siteKey` parameter.
    ///
    /// The captcha result of the client can either be passed into the repeated original request
    /// with the `X-Captcha-Result` header or alternatively to the `POST /captcha/solve` endpoint.
    /// The time a solved captcha is remembered varies across different endpoints and can also be
    /// influenced by individual client behavior.
    ///
    /// Authentication is not required for the `POST /captcha/solve` endpoint, captchas are tracked
    /// both by client ip and logged in user id. If you are logged in, you want to send the session
    /// token along, so you validate the captcha for your client ip and user id at the same time,
    /// but it is not required.
    // TODO: Use enum representations once the structure of this field is known.
    // See: https://serde.rs/enum-representations.html
    pub context: Option<HashMap<String, String>>,
}
