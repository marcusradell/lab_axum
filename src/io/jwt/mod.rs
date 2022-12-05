use crate::result::Result;
use claims::Claims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub mod claims;

pub struct Jwt {
    encodingKey: EncodingKey,
    decodingKey: DecodingKey,
}

impl Jwt {
    pub fn new(secret: &str) -> Self {
        Self {
            encodingKey: EncodingKey::from_secret(secret.as_ref()),
            decodingKey: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn encode(&self, claims: &Claims) -> Result<String> {
        let token = encode(&Header::default(), claims, &self.encodingKey)?;

        Ok(token)
    }

    pub fn decode(&self, token: &str) -> Result<Claims> {
        let token = decode::<Claims>(&token, &self.decodingKey, &Validation::default())?;

        Ok(token.claims)
    }
}
