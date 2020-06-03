use super::dao::IUser;
use super::user::{Claims, Login, Register};
use crate::api::ApiResult;
use crate::middlewares::auth::AuthorizationService;
use crate::state::AppState;

use actix_web::{get, post, web, HttpRequest, Responder};

// curl --data '{"name": "avx", "email": "avx@spark.com", "password": "avx512sha"}' -H "Content-Type: application/json" -X POST localhost:8080/user/register
#[post("/register")]
async fn register(form: web::Json<Register>, state: AppState) -> impl Responder {
    let form = form.into_inner();

    match state.user_add(&form).await {
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

// curl --data '{"name": "avx", "email": "avx@spark.com", "password": "avx512sha"}' -H "Content-Type: application/json" -X POST localhost:8080/user/login
#[post("/login")]
async fn login(form: web::Json<Login>, state: AppState) -> impl Responder {
    let form = form.into_inner();

    use chrono::{DateTime, Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};

    match state.user_query(&form.name).await {
        Ok(user) => {
            info!("find user {:?} ok: {:?}", form, user);

            if form.verify(&user.pass) {
                let exp: DateTime<Utc> = Utc::now()
                    + if form.rememberme {
                        Duration::days(32)
                    } else {
                        Duration::hours(2)
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

#[get("/userInfo")]
async fn user_informations(
    _req: HttpRequest,
    auth: AuthorizationService,
    state: AppState,
) -> impl Responder {
    match state.user_query(&auth.claims.sub).await {
        Ok(user) => {
            debug!("find user {:?} ok: {:?}", auth.claims, user);
            ApiResult::new().with_data(user)
        }
        Err(e) => {
            error!("find user {:?} error: {:?}", auth.claims, e);
            ApiResult::new().code(500).with_msg(e.to_string())
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(user_informations);
}
