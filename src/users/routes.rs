use super::dao::IUser;
use super::user::{Claims, Login, Register, UserAddress};
use crate::api::ApiResult;
use crate::middlewares::auth::AuthorizationService;
use crate::state::AppState;

use actix_web::{delete as del, get, post, web, Responder};
use validator::Validate;

// curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/user/register
#[post("/register")]
async fn register(form: web::Json<Register>, state: AppState) -> impl Responder {
    let form = form.into_inner();

    if let Err(e) = form.validate() {
        error!("regitser {:?} error: {:?}", form, e);
        return ApiResult::new().code(400).with_msg(e.to_string());
    }

    match state.get_ref().user_add(&form).await {
        Ok(res) => {
            info!("register {:?} res: {}", form, res);
            ApiResult::new().with_msg("ok").with_data(res)
        }
        Err(e) => {
            error!("regitser {:?} error: {:?}", form, e);
            ApiResult::new().code(400).with_msg(e.to_string())
        }
    }
}

// curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:8080/user/login
#[post("/login")]
async fn login(form: web::Json<Login>, state: AppState) -> impl Responder {
    let form = form.into_inner();

    use chrono::{DateTime, Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};

    // todo: distable login for deleted and blocked users
    match state.get_ref().user_query(&form.name).await {
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
                let key = state.config.jwt_priv.as_bytes();
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
    }
}

// curl -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJCb2IiLCJleHAiOjE1OTEyNDYwOTR9.O1dbYu3tqiIi6I8OUlixLuj9dp-1tLl4mjmXZ0ve6uo' localhost:8080/user/info/who |jq .
// curl 'localhost:8080/user/userInfo?access_token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJCb2IiLCJleHAiOjE1OTEyNTYxNDd9.zJKlZOozYfq-xMXO89kjUyme6SA8_eziacqt5gvXj2U' |jq .
#[get("/info/{who}")]
async fn info(
    form: web::Path<String>,
    auth: AuthorizationService,
    state: AppState,
) -> impl Responder {
    let who = form.into_inner();
    let w = who.as_str();

    // me
    let user = match state.get_ref().user_query(&auth.claims.sub).await {
        Ok(user) => {
            debug!("find user {:?} ok: {:?}", auth.claims, user);

            if who == "_"
                || [
                    user.id.to_string().as_str(),
                    user.name.as_str(),
                    user.email.as_str(),
                ]
                .contains(&w)
            {
                return ApiResult::new().with_msg("ok").with_data(user);
            }

            user
        }
        Err(e) => {
            error!("find user {:?} error: {:?}", auth.claims, e);
            return ApiResult::new().code(500).with_msg(e.to_string());
        }
    };

    // todo: add role(admin, user, guest)
    if user.status != "normal" {
        return ApiResult::new().code(403);
    }

    match state.get_ref().user_query(w).await {
        Ok(user) => {
            debug!("find user {:?} ok: {:?}", w, user);
            ApiResult::new().with_msg("ok").with_data(user)
        }
        Err(e) => {
            error!("find user {:?} error: {:?}", w, e);
            ApiResult::new().code(500).with_msg(e.to_string())
        }
    }
}

// curl -v -X DELETE localhost:8080/user/who
#[del("/delete/{who}")]
async fn delete(
    form: web::Path<String>,
    auth: AuthorizationService,
    state: AppState,
) -> impl Responder {
    let user = match state.get_ref().user_query(&auth.claims.sub).await {
        Ok(user) => user,
        Err(e) => {
            error!("find user {:?} error: {:?}", auth.claims, e);
            return ApiResult::new().code(500).with_msg(e.to_string());
        }
    };

    // todo: add role(admin, user, guest)
    if user.status != "normal" {
        return ApiResult::new().code(403);
    }

    let who = form.into_inner();
    match state.get_ref().user_delete(&who).await {
        Ok(res) => {
            info!(
                "delete {:?} res: {} {} {} {}",
                who, res.id, res.name, res.email, res.status
            );
            ApiResult::new().with_msg("ok").with_data(res)
        }
        Err(e) => {
            error!("delete {:?} error: {:?}", who, e);
            ApiResult::new().code(400).with_msg(e.to_string())
        }
    }
}

#[post("/address")]
async fn user_address(form: web::Json<UserAddress>, state: AppState) -> impl Responder {
    let form = form.into_inner();
    match state.get_ref().adress_query(&form.share_address.clone()).await {
        Ok(addex) => {
            let mut addres :i16 = addex.experience.parse::<i16>().unwrap();
            addres = addres + 1;
            match state.get_ref().adress_update(&form.share_address.clone(), &addres.to_string()).await {
                Ok(res) => {
                    ApiResult::new().with_msg("ok").with_data(res)
                }
                Err(e) => {
                    ApiResult::new().code(400).with_msg(e.to_string())
                }
            };
        },
        Err(_) => {
            print!("no find address");
        }
    };

    match state.get_ref().adress_add(&form.share_address, "1").await {
        Ok(res) => {
            ApiResult::new().with_msg("ok").with_data(res)
        }
        Err(e) => {
            ApiResult::new().code(400).with_msg(e.to_string())
        }
    }
}
/*
#[post("/email")]
async fn post_email(form: web::Json<Email>, state: AppState) -> impl Responder {
    let form = form.into_inner();

    use chrono::{DateTime, Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};

    match state.get_ref().user_query(&form.email_address).await {
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
                let key = state.config.jwt_priv.as_bytes();
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
    }
}
*/

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(delete);
    cfg.service(info);
    cfg.service(user_address);
}
