use jsonwebtoken::{encode, Header, Algorithm, EncodingKey, DecodingKey, decode, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use entities::sea_orm_active_enums::Role as UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    user_role: Option<UserRole>,
    exp: i64
}

impl Claims {
    pub fn new(user_id: Uuid, user_role: Option<UserRole>) -> Self {
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::hours(1)).timestamp();
        
        Claims {
            exp,
            sub: user_id,
            user_role
        }
    }

    pub fn new_with_custom_time(user_id: Uuid, user_role: Option<UserRole>, exp_time: i64) -> Self {
        Claims {
            exp: exp_time,
            sub: user_id,
            user_role
        }
    }
}

#[derive(Debug)]
pub struct EncodedToken {
    pub token: String,
}

#[derive(Debug)]
pub struct DecodedToken {
    pub user_id: Uuid,
    pub user_role: Option<UserRole>,
    pub exp: i64
}

#[derive(Debug)]
pub struct MakeJwtResult {
    pub access_token: EncodedToken,
    pub refresh_token: EncodedToken
}

pub struct JwtService {}

impl JwtService {
    pub fn make_jwt(&self, user_id: uuid::Uuid, user_role: UserRole, encoding_key: EncodingKey) ->  Result<MakeJwtResult, JwtError>{
        let mut header: Header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());

        let access_claims: Claims = Claims::new(user_id, Some(user_role));

        let refresh_token_lifetime: i64 = (chrono::Utc::now() + chrono::Duration::hours(5)).timestamp();

        let refresh_claims: Claims = Claims::new_with_custom_time(user_id, None, refresh_token_lifetime);

        let access_token: Result<String, JwtError> = encode(&header, &access_claims, &encoding_key);
        let refresh_token: Result<String, JwtError> = encode(&header, &refresh_claims, &encoding_key);

        drop(encoding_key);

        if access_token.is_err() || refresh_token.is_err() {
            return Err(access_token.unwrap_err());
        }
        
        Ok(MakeJwtResult {
            access_token: EncodedToken { token: access_token.unwrap() },
            refresh_token: EncodedToken { token: refresh_token.unwrap() }
        })
    }

    pub fn decode_jwt(&self, token: String, decoding_key: DecodingKey) -> Result<DecodedToken, JwtError> {
        let mut header: Header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());

        let validation: Validation = Validation::new(Algorithm::HS256);

        let token: Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> = decode::<Claims>(&token, &decoding_key, &validation);
        
        drop(decoding_key);

        match token {
            Ok(token) => {
                let id = token.claims.sub;
                let role = token.claims.user_role;

                Ok(DecodedToken {
                    user_id: id,
                    exp: token.claims.exp,
                    user_role: role
                })
            },
            Err(err) => Err(err)
        }
    }
}