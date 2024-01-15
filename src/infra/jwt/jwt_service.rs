use jsonwebtoken::{encode, Header, Algorithm, EncodingKey, DecodingKey, decode, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    exp: i64
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::minutes(5)).timestamp();
        
        Claims {
            exp,
            sub: user_id
        }
    }

    pub fn new_with_custom_time(user_id: Uuid, exp_time: i64) -> Self {
        Claims {
            exp: exp_time,
            sub: user_id
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
    pub exp: i64
}

#[derive(Debug)]
pub struct MakeJwtResult {
    pub access_token: EncodedToken,
    pub refresh_token: EncodedToken
}

pub struct JwtService {}

impl JwtService {
    pub fn make_jwt(user_id: uuid::Uuid, encoding_key: EncodingKey) ->  Result<MakeJwtResult, JwtError>{
        let mut header: Header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());

        let access_claims: Claims = Claims::new(user_id);

        let one_hour: i64 = (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp();

        let refresh_claims: Claims = Claims::new_with_custom_time(user_id, one_hour);

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

    pub fn decode_jwt(token: String, decoding_key: DecodingKey) -> Result<DecodedToken, JwtError> {
        let mut header: Header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());

        let validation: Validation = Validation::new(Algorithm::HS256);

        let token: Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> = decode::<Claims>(&token, &decoding_key, &validation);
        
        drop(decoding_key);

        match token {
            Ok(token) => {
                let id = token.claims.sub;

                Ok(DecodedToken { user_id: id, exp: token.claims.exp })
            },
            Err(err) => Err(err)
        }
    }
}