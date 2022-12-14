use crate::domains::identities::Role;
use crate::result::Result;
use chrono::{Duration, Utc};
use claims::Claims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub mod claims;

#[derive(Clone)]
pub struct Jwt {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Jwt {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
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

    pub fn decode(&self, token: &str) -> Result<Claims> {
        let token = decode::<Claims>(&token, &self.decoding_key, &Validation::default())?;

        Ok(token.claims)
    }
}
