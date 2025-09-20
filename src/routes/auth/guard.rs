use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use poem::{FromRequest, Request, RequestBody};
use secrecy::ExposeSecret;
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize, Debug)]
struct SupabaseClaims {
    pub sub: String,
}

pub struct AuthenticatedUser {
    pub user_id: String,
    pub token: String,
}

// Poem runs this extractor automatically whenever a handler declares an `AuthenticatedUser`
// parameter, so routes just add the argument and receive a validated Supabase user id.
impl<'a> FromRequest<'a> for AuthenticatedUser {
    async fn from_request(req: &'a Request, _: &mut RequestBody) -> poem::Result<Self> {
        let token = req
            .headers()
            .get(poem::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| poem::Error::from_status(poem::http::StatusCode::UNAUTHORIZED))?;

        let raw_token = token.to_string();

        let state = req.data::<AppState>().ok_or_else(|| {
            poem::Error::from_status(poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;

        let secret = state.supabase_jwt_secret.expose_secret();
        let decoding_key = DecodingKey::from_base64_secret(secret)
            .unwrap_or_else(|_| DecodingKey::from_secret(secret.as_bytes()));

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_aud = false;

        let data = decode::<SupabaseClaims>(&raw_token, &decoding_key, &validation)
            .map_err(|_| poem::Error::from_status(poem::http::StatusCode::UNAUTHORIZED))?;

        Ok(Self {
            user_id: data.claims.sub,
            token: raw_token,
        })
    }
}
