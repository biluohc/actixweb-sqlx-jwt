use serde::Serialize;
use std::borrow::Cow;

use actix_web::{error, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResult<T = ()> {
    pub code: i32,
    pub msg: Option<Cow<'static, str>>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
    pub fn new() -> Self {
        Self {
            code: 200,
            msg: None,
            data: None,
        }
    }
    pub fn code(mut self, code: i32) -> Self {
        self.code = code;
        self
    }
    pub fn with_msg<S: Into<Cow<'static, str>>>(mut self, msg: S) -> Self {
        self.msg = Some(msg.into());
        self
    }
    pub fn msg_as_str(&self) -> &str {
        self.msg.as_ref().map(|s| s.as_ref()).unwrap_or_default()
    }
    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
    pub fn to_resp(&self, req: &HttpRequest) -> HttpResponse {
        info!(
            "{} \"{} {} {:?}\" {}",
            req.peer_addr().unwrap(),
            req.method(),
            req.uri(),
            req.version(),
            self.code
        );

        let resp = match serde_json::to_string(self) {
            Ok(json) => HttpResponse::Ok()
                .content_type("application/json")
                .body(json),
            Err(e) => Error::from(e).into(),
        };

        resp
    }
}

// Either and AsRef/Responder not in crate
pub enum ApiRt<L, R> {
    Ref(L),
    T(R),
}

impl<T, R> Responder for ApiRt<R, ApiResult<T>>
where
    T: Serialize,
    R: AsRef<ApiResult<T>>,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        match self {
            ApiRt::Ref(a) => a.as_ref().respond_to(req),
            ApiRt::T(b) => b.respond_to(req),
        }
    }
}

impl<T: Serialize> Responder for ApiResult<T> {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        (&self).respond_to(req)
    }
}
impl<T: Serialize> Responder for &ApiResult<T> {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        ready(Ok(self.to_resp(req)))
    }
}

// return 200 all
pub fn json_error_handler<E: std::fmt::Display + std::fmt::Debug + 'static>(
    err: E,
    req: &HttpRequest,
) -> error::Error {
    let detail = err.to_string();
    let api = ApiResult::new().with_data(()).code(400).with_msg(detail);
    let response = api.to_resp(req);

    error::InternalError::from_response(err, response).into()
}

pub async fn notfound(req: HttpRequest) -> Result<HttpResponse, Error> {
    let api = ApiResult::new()
        .with_data(())
        .code(404)
        .with_msg("route not found");

    api.respond_to(&req).await
}
