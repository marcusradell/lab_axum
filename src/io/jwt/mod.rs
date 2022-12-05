use jsonwebtoken::EncodingKey;

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
}
