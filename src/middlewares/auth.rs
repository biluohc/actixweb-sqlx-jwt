use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::dev::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{err, ok, Ready};
use futures::Future;

use crate::state::AppStateRaw;
use crate::users::user::Claims;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub struct JwtAuth;

impl<S, B> Transform<S> for JwtAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware { service })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for JwtAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let claims = req
            .headers()
            .get("Authorization")
            .and_then(|au| au.to_str().ok())
            .and_then(|au| {
                let words = au.split("Bearer").collect::<Vec<&str>>();
                let token = words.get(1).map(|w| w.trim());
                info!("au: {} -> {:?}", au, token);
                token
            })
            .and_then(|token| {
                let state = req.app_data::<AppStateRaw>().expect("get AppStateRaw");
                let key = state.config.jwt_priv.as_bytes();

                decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS256),
                )
                .map_err(|e| error!("jwt.decode {} failed: {:?}", token, e))
                .ok()
            });

        info!("claims: {:?}", claims);
        if claims.is_some() {
            req.extensions_mut().deref_mut().insert(true);
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

//前面的中间价得放到路由前，不方便处理个别路径，后者放在请求预处理，貌似还可以

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, FromRequest, HttpRequest};

#[derive(Debug)]
pub struct AuthorizationService {
    token: String,
    pub claims: Claims,
}

// curl --data '{"name": "avx", "email": "avx@spark.com", "password": "avx512sha"}' -H "Content-Type: application/json" -X POST localhost:8080/user/login                                  (310ms)
// 和示例饿的split好像不太一致: {"code":0,"msg":"ok","data":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhdnhAc3BhcmsuY29tIiwiZXhwIjoxNTkwNTAxODA3fQ.GJNcOvkoJQvQPJywBvC8fRhpo0igHpMz1yLKC3Fq6Bw"}
impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|au| au.to_str().ok())
            .and_then(|au| {
                let words = au.split("Bearer").collect::<Vec<&str>>();
                let token = words.get(1).map(|w| w.trim());
                info!("au: {} -> {:?}", au, token);
                token
            });

        if let Some(token) = token {
            let state = req.app_data::<AppStateRaw>().expect("get AppStateRaw");
            let key = state.config.jwt_priv.as_bytes();

            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(key),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(claims) => ok(AuthorizationService {
                    token: token.to_owned(),
                    claims: claims.claims,
                }),
                Err(e) => {
                    error!("jwt.decode {} failed: {:?}", token, e);
                    err(ErrorUnauthorized("invalid token"))
                }
            }
        } else {
            err(ErrorUnauthorized("Unauthorized"))
        }
    }
}
