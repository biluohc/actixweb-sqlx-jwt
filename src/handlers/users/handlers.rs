use super::dao::IUser;
use crate::api::ApiResult;
use crate::middlewares::auth::AUTH;
use crate::models::user::{Claims, Login, Register};
use crate::state::global_state;

use salvo::prelude::*;

// curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/api/user/register
#[fn_handler]
pub async fn register(req: &mut Request, res: &mut Response) {
    if let Ok(form) = req
        .read::<Register>()
        .await
        .map_err(|_| res.set_status_code(StatusCode::BAD_REQUEST))
    {
        let api = match global_state().user_add(&form).await {
            Ok(res) => {
                info!("register {:?} res: {}", form, res);
                ApiResult::new().with_msg("ok").with_data(res)
            }
            Err(e) => {
                error!("regitser {:?} error: {:?}", form, e);
                ApiResult::new().code(400).with_msg(e.to_string())
            }
        };

        api.apply_to_res(res);
    }
}

// curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/api/user/login
#[fn_handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    if let Ok(form) = req
        .read::<Login>()
        .await
        .map_err(|_| res.set_status_code(StatusCode::BAD_REQUEST))
    {
        use chrono::{DateTime, Duration, Utc};
        use jsonwebtoken::{encode, EncodingKey, Header};

        let api = match global_state().user_query(&form.name).await {
            Ok(user) => {
                info!("find user {:?} ok: {:?}", form, user);

                if form.verify(&user.pass) {
                    let exp: DateTime<Utc> = Utc::now()
                        + if form.rememberme {
                            Duration::days(30)
                        } else {
                            Duration::hours(1)
                        };

                    let my_claims = Claims {
                        sub: user.name,
                        exp: exp.timestamp() as usize,
                    };
                    let key = global_state().config.jwt_priv.as_bytes();
                    let token = encode(
                        &Header::default(),
                        &my_claims,
                        &EncodingKey::from_secret(key),
                    )
                    .unwrap();

                    ApiResult::new().with_msg("ok").with_data(token)
                } else {
                    ApiResult::new().code(403).with_msg("wrong pass or name")
                }
            }
            Err(e) => {
                error!("find user {:?} error: {:?}", form, e);
                ApiResult::new().code(400).with_msg(e.to_string())
            }
        };

        api.apply_to_res(res);
    }
}

// curl -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJCb2IiLCJleHAiOjE1OTEyNDYwOTR9.O1dbYu3tqiIi6I8OUlixLuj9dp-1tLl4mjmXZ0ve6uo' localhost:8080/api/user/info |jq .
// curl 'localhost:8080/api/user/info?access_token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJCb2IiLCJleHAiOjE1OTEyNTYxNDd9.zJKlZOozYfq-xMXO89kjUyme6SA8_eziacqt5gvXj2U' |jq .
#[fn_handler]
pub async fn user_info(depot: &mut Depot, res: &mut Response) {
    let auth: &Claims = depot.borrow(AUTH);

    let api = match global_state().user_query(&auth.sub).await {
        Ok(user) => {
            debug!("find user {:?} ok: {:?}", auth.sub, user);
            ApiResult::new().with_data(user)
        }
        Err(e) => {
            error!("find user {:?} error: {:?}", auth, e);
            ApiResult::new().code(500).with_msg(e.to_string())
        }
    };

    api.apply_to_res(res);
}
