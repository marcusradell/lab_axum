use crate::result::Result;
use claims::Claims;
use jsonwebtoken::{encode, EncodingKey, Header};

pub mod claims;

pub struct Jwt {
    encodingKey: EncodingKey,
}

impl Jwt {
    pub fn new(secret: &str) -> Self {
        Self {
            encodingKey: EncodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn encode(&self, claims: &Claims) -> Result<String> {
        let token = encode(&Header::default(), claims, &self.encodingKey)?;

        Ok(token)
    }
}
