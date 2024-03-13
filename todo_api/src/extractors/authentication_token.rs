use std::future::{ready, Ready};

use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
use actix_web::{web, Error, FromRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Claims;

pub struct AuthenticationToken {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

impl FromRequest for AuthenticationToken {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        //Get auth token from the authentication header
        let auth_token = match req.cookie("todo_auth") {
            Some(cookie) => cookie.value().to_owned(),
            None => return ready(Err(ErrorBadRequest("No token provided"))),
        };

        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized("Invalid auth token")));
        }

        let secret = req.app_data::<web::Data<String>>().unwrap();

        //Decode token
        let decode = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        );

        //Return token
        match decode {
            Ok(token) => {
                return ready(Ok(AuthenticationToken {
                    user_id: token.claims.user_id,
                }))
            }
            Err(_) => return ready(Err(ErrorUnauthorized("Unauthorized"))),
        }
    }
}
