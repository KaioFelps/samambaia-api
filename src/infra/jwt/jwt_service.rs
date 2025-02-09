use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::domain_entities::role::Role;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    user_role: Option<Role>,
    exp: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, user_role: Option<Role>) -> Self {
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::try_hours(1).unwrap()).timestamp();

        Claims {
            exp,
            sub: user_id,
            user_role,
        }
    }

    pub fn new_with_custom_time(user_id: Uuid, user_role: Option<Role>, exp_time: i64) -> Self {
        Claims {
            exp: exp_time,
            sub: user_id,
            user_role,
        }
    }
}

#[derive(Debug)]
pub struct EncodedToken {
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct DecodedToken {
    pub user_id: Uuid,
    pub user_role: Option<Role>,
    pub exp: i64,
}

#[derive(Debug)]
pub struct MakeJwtResult {
    pub access_token: EncodedToken,
    pub refresh_token: EncodedToken,
}

pub struct JwtService;

impl JwtService {
    pub fn make_jwt(
        &self,
        user_id: uuid::Uuid,
        user_role: Role,
        encoding_key: EncodingKey,
    ) -> Result<MakeJwtResult, JwtError> {
        let mut header: Header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());

        let access_claims: Claims = Claims::new(user_id, Some(user_role.clone()));

        let refresh_token_lifetime: i64 =
            (chrono::Utc::now() + chrono::Duration::try_hours(5).unwrap()).timestamp();

        let refresh_claims: Claims =
            Claims::new_with_custom_time(user_id, Some(user_role), refresh_token_lifetime);

        let access_token = encode(&header, &access_claims, &encoding_key);
        let refresh_token = encode(&header, &refresh_claims, &encoding_key);

        drop(encoding_key);

        Ok(MakeJwtResult {
            access_token: EncodedToken {
                token: access_token?,
            },
            refresh_token: EncodedToken {
                token: refresh_token?,
            },
        })
    }

    pub fn decode_jwt(
        &self,
        token: String,
        decoding_key: DecodingKey,
    ) -> Result<DecodedToken, JwtError> {
        let mut header: Header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());

        let validation: Validation = Validation::new(Algorithm::HS256);

        let token: Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> =
            decode::<Claims>(&token, &decoding_key, &validation);

        drop(decoding_key);

        match token {
            Ok(token) => {
                let id = token.claims.sub;
                let role = token.claims.user_role;

                Ok(DecodedToken {
                    user_id: id,
                    exp: token.claims.exp,
                    user_role: role,
                })
            }
            Err(err) => Err(err),
        }
    }
}
