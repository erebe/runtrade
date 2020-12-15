use actix_web::{FromRequest, Error, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use std::future::Ready;
use actix_web::dev::Payload;
use actix_web::http::header::AUTHORIZATION;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize};
use uuid::Uuid;

lazy_static! {
    pub static ref JWK_E: String = std::env::var("JWK_E").expect("Missing JWK_E env");
    pub static ref JWK_N: String = std::env::var("JWK_N").expect("Missing JWK_N env");
}

#[derive(Deserialize, Debug)]
pub struct Authorized {
   pub acr: String,
   #[serde(alias = "allowed-origins")]
   pub allowed_origins: Vec<String>,
   pub azp: String,
   pub email: String,
   pub email_verified: bool,
   pub exp: u64,
   pub family_name: String,
   pub given_name: String,
   pub iat: u64,
   pub iss: String,
   pub jti: String,
   pub name: String,
   pub nonce: String,
   pub preferred_username: String,
   pub sub: Uuid,
}

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        std::future::ready(is_authorized(req))
    }
}

fn is_authorized(req: &HttpRequest) -> Result<Authorized, actix_web::Error> {

    // PUT OR DELETE methods
    let auth_header = req.headers()
        .get(AUTHORIZATION)
        .ok_or(ErrorUnauthorized("Not Authorized"))?
        .to_str()
        .or( Err( ErrorUnauthorized("Not Authorized")))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(ErrorUnauthorized("Bearer token required"));

    }

    // let token = decode::<Value>(&auth_header[7..],
    //                                  &DecodingKey::from_rsa_components(JWK_N.as_str(), JWK_E.as_str()),
    //                                  &Validation::new(Algorithm::RS256));
    // dbg!(token);
    let token = decode::<Authorized>(&auth_header[7..],
                                     &DecodingKey::from_rsa_components(JWK_N.as_str(), JWK_E.as_str()),
                                     &Validation::new(Algorithm::RS256))
        .or(Err(ErrorUnauthorized("Invalid bearer token")))?;

    Ok(token.claims)
}
