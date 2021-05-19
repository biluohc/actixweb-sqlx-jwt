use serde::Serialize;
use std::borrow::Cow;

pub use salvo::http::{header, Mime, Request, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResult<T = ()> {
    pub code: u16,
    pub msg: Option<Cow<'static, str>>,
    pub data: Option<T>,
}

pub type ApiError = ApiResult<()>;

impl<T: Serialize> ApiResult<T> {
    pub fn new() -> Self {
        Self {
            code: 200,
            msg: None,
            data: None,
        }
    }
    pub fn code(mut self, code: u16) -> Self {
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

    pub fn apply_to_res(&self, res: &mut Response) {
        let mime = "application/json; charset=utf-8"
            .parse()
            .expect("default mime invalid");
        res.headers_mut().insert(header::CONTENT_TYPE, mime);
        let json = serde_json::to_string(&self).expect("ApiResult to json");
        // info!("json: {}", json);

        let mut code = self.code;
        loop {
            if code >= 1000 {
                code /= 10;
            } else {
                break;
            }
        }
        res.set_status_code(StatusCode::from_u16(code).expect("StatusCode invalid"));
        res.render_json_text(json.as_str());
    }
}

use std::fmt::{self, Debug, Display};
impl<T: Debug + Serialize> Display for ApiResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[async_trait]
impl<T: Serialize + Send> salvo::Writer for ApiResult<T> {
    async fn write(mut self, _req: &mut Request, _depot: &mut salvo::Depot, res: &mut Response) {
        self.apply_to_res(res)
    }
}
