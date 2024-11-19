use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::constants::config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iat: usize, // Token generatd DateTime in Miliseconds
    pub exp: usize, // Expiry DateTime in Miliseconds
    pub subject: String,
    pub id: String,
}

pub fn encode_jwt(email: String, id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(1)).timestamp() as usize;

    let claims = Claims {
        iat,
        exp,
        subject: email,
        id,
    };

    let secret = (config::SECRET_KEY).clone();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_jwt(jwt: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = (config::SECRET_KEY).clone();
    let     claim_data = decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;
    log::info!("Decoded JWT: {:?}", claim_data);
    Ok(claim_data.claims)
}
