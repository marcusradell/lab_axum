use crate::result::Result;
use crate::services::identities::Role;
use chrono::{Duration, Utc};
use claims::Claims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub mod claims;

#[derive(Clone)]
pub struct Jwt {
    encoding_key: EncodingKey,
    _decoding_key: DecodingKey,
}

impl Jwt {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            _decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn encode(&self, id: &str, role: &Role) -> Result<String> {
        let claims = Claims {
            sub: id.to_string(),
            role: role.to_string(),
            exp: (Utc::now() + Duration::weeks(52)).timestamp().try_into()?,
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)?;

        Ok(token)
    }

    pub fn _decode(&self, token: &str) -> Result<Claims> {
        let token = decode::<Claims>(token, &self._decoding_key, &Validation::default())?;

        Ok(token.claims)
    }
}
