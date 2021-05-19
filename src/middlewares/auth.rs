use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use salvo::prelude::*;
use std::borrow::Cow;

use crate::api::ApiError;
use crate::models::user::Claims;
use crate::state::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct QueryParams {
    access_token: String,
}

pub static AUTH: &str = "auth";

// 1. header: Authorization: Bearer xxx
// 2. URL's query: ?access_token=xxx
// 3x. Body's query: ?access_token=xxx
#[fn_handler]
pub async fn auth_handler(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
            let query = req.uri().query().unwrap_or_default();
            let token = serde_qs::from_str::<QueryParams>(query);
            debug!("JWT.access_token: {} -> {:?}", query, token);
            token.map(|p| p.access_token.into()).ok()
        });

    match token
        .as_ref()
        .ok_or_else(|| Cow::Borrowed("Unauthorized"))
        .and_then(|token| {
            let state = global_state();
            let key = state.config.jwt_priv.as_bytes();
            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(key),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(claims) => Ok(claims.claims),
                Err(e) => {
                    error!("jwt.decode {} failed: {:?}", token, e);
                    Err(format!("invalid token: {}", e).into())
                }
            }
        }) {
        Ok(claims) => depot.insert(AUTH, claims),
        Err(e) => {
            let api = ApiError::new().code(400).with_msg(e);
            api.apply_to_res(res);
            res.commit();
        }
    }
}
