use std::hash::{Hash, Hasher};

use secrecy::{ExposeSecret, SecretString};
use validator::{Validate, ValidateEmail};

use crate::domain::error::app_error::{AppResult, ValidationError};

#[derive(Debug, Clone)]
pub struct Email {
    inner: SecretString,
}

impl AsRef<SecretString> for Email {
    fn as_ref(&self) -> &SecretString {
        &self.inner
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.inner.expose_secret() == other.inner.expose_secret()
    }
}

impl Hash for Email {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.expose_secret().hash(state);
    }
}

impl Eq for Email {}

impl Validate for Email {
    #[tracing::instrument(name = "email_validation", skip_all)]
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        if !self.inner.expose_secret().validate_email() {
            return Err(validator::ValidationErrors::new());
        }

        Ok(())
    }
}

impl Email {
    #[tracing::instrument(name = "email_creation", skip_all)]
    pub fn new(email: String) -> AppResult<Self> {
        let email = Email {
            inner: SecretString::from(email),
        };
        email
            .validate()
            .map_err(|_| ValidationError::InvalidEmail)?;
        Ok(email)
    }
}
