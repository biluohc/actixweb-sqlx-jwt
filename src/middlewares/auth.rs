use actix_web::{dev, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::borrow::Cow;

use crate::api::ApiError;
use crate::state::AppStateRaw;
use crate::users::user::Claims;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct QueryParams {
    access_token: String,
}

#[derive(Debug)]
pub struct AuthorizationService {
    pub claims: Claims,
}

impl FromRequest for AuthorizationService {
    type Error = ApiError;
    type Future = Ready<Result<AuthorizationService, Self::Error>>;
    type Config = ();

    // 1. header: Authorization: Bearer xxx
    // 2. URL's query: ?access_token=xxx
    // 3x. Body's query: ?access_token=xxx
    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                let words = h.split("Bearer").collect::<Vec<&str>>();
                let token = words.get(1).map(|w| w.trim());
                debug!("JWT.Authorization: {} -> {:?}", h, token);
                token.map(|t| Cow::Borrowed(t))
            })
            .or_else(|| {
                let query = req.query_string();
                let token = serde_qs::from_str::<QueryParams>(query);
                debug!("JWT.access_token: {} -> {:?}", query, token);
                token.map(|p| p.access_token.into()).ok()
            });

        match token
            .as_ref()
            .ok_or_else(|| Cow::Borrowed("Unauthorized"))
            .and_then(|token| {
                let state = req.app_data::<AppStateRaw>().expect("get AppStateRaw");
                let key = state.config.jwt_priv.as_bytes();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(claims) => Ok(AuthorizationService {
                        claims: claims.claims,
                    }),
                    Err(e) => {
                        error!("jwt.decode {} failed: {:?}", token, e);
                        Err(format!("invalid token: {}", e).into())
                    }
                }
            }) {
            Ok(service) => ok(service),
            Err(e) => {
                let api = ApiError::new().code(400).with_msg(e);
                api.log(req);
                err(api)
            }
        }
    }
}
