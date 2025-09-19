use std::hash::{Hash, Hasher};

use secrecy::{ExposeSecret, SecretString};
use validator::Validate;

use crate::domain::error::app_error::{AppResult, ValidationError};

#[derive(Debug, Clone)]
pub struct Password {
    inner: SecretString,
}

impl AsRef<SecretString> for Password {
    fn as_ref(&self) -> &SecretString {
        &self.inner
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.inner.expose_secret() == other.inner.expose_secret()
    }
}

impl Hash for Password {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.expose_secret().hash(state);
    }
}

impl Eq for Password {}

impl Validate for Password {
    #[tracing::instrument(name = "password_validation", skip_all)]
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        let password = self.inner.expose_secret();

        // Password must be at least 8 characters long
        if password.len() < 8 {
            return Err(validator::ValidationErrors::new());
        }

        // Must contain at least 1 number
        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(validator::ValidationErrors::new());
        }

        // Must contain at least 1 uppercase letter
        if !password.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(validator::ValidationErrors::new());
        }

        // Must contain at least 1 special character (non-alphanumeric)
        if !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(validator::ValidationErrors::new());
        }

        Ok(())
    }
}

impl Password {
    #[tracing::instrument(name = "password_creation", skip_all)]
    pub fn new(password: String) -> AppResult<Self> {
        let password = Password {
            inner: SecretString::from(password),
        };
        password
            .validate()
            .map_err(|_| ValidationError::WeakPassword)?;
        Ok(password)
    }
}
