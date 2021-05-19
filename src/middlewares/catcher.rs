use crate::api::ApiResult;
use salvo::http::errors::*;
pub use salvo::http::{guess_accept_mime, header, Mime, Request, Response, StatusCode};
pub use salvo::Catcher;

/// Default implemention of Catcher.
pub struct CatcherImpl(HttpError);
impl CatcherImpl {
    pub fn new(e: HttpError) -> CatcherImpl {
        CatcherImpl(e)
    }
}
// https://github.com/salvo-rs/salvo/issues/29
impl Catcher for CatcherImpl {
    fn catch(&self, req: &Request, res: &mut Response) -> bool {
        let status = res.status_code().unwrap_or(StatusCode::NOT_FOUND);
        if status != self.0.code {
            return false;
        }

        // application/json; charset=utf-8
        if req.uri().path().starts_with("/api") {
            if res.body().is_none() {
                let api = ApiResult::new()
                    .with_data(())
                    .code(self.0.code.as_u16() as _)
                    .with_msg(self.0.code.canonical_reason().unwrap_or_default());

                api.apply_to_res(res);
            }
        } else {
            let mime = guess_accept_mime(req, None);
            let (format, data) = &self.0.as_bytes(&mime);
            res.headers_mut()
                .insert(header::CONTENT_TYPE, format.to_string().parse().unwrap());
            res.write_body_bytes(&data);
        }

        true
    }
}

macro_rules! default_catchers {
    ($($code:expr),+) => (
        let list: Vec<Box<dyn Catcher>> = vec![
        $(
            Box::new(CatcherImpl::new(salvo::http::errors::http_error::from_code($code).unwrap())),
        )+];
        list
    )
}

pub mod defaults {
    use super::{Catcher, CatcherImpl, StatusCode};

    pub fn get() -> Vec<Box<dyn Catcher>> {
        default_catchers! {
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::PAYMENT_REQUIRED,
            StatusCode::FORBIDDEN,
            StatusCode::NOT_FOUND,
            StatusCode::METHOD_NOT_ALLOWED,
            StatusCode::NOT_ACCEPTABLE,
            StatusCode::PROXY_AUTHENTICATION_REQUIRED,
            StatusCode::REQUEST_TIMEOUT,
            StatusCode::CONFLICT,
            StatusCode::GONE,
            StatusCode::LENGTH_REQUIRED,
            StatusCode::PRECONDITION_FAILED,
            StatusCode::PAYLOAD_TOO_LARGE,
            StatusCode::URI_TOO_LONG,
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            StatusCode::RANGE_NOT_SATISFIABLE,
            StatusCode::EXPECTATION_FAILED,
            StatusCode::IM_A_TEAPOT,
            StatusCode::MISDIRECTED_REQUEST,
            StatusCode::UNPROCESSABLE_ENTITY,
            StatusCode::LOCKED,
            StatusCode::FAILED_DEPENDENCY,
            StatusCode::UPGRADE_REQUIRED,
            StatusCode::PRECONDITION_REQUIRED,
            StatusCode::TOO_MANY_REQUESTS,
            StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::NOT_IMPLEMENTED,
            StatusCode::BAD_GATEWAY,
            StatusCode::SERVICE_UNAVAILABLE,
            StatusCode::GATEWAY_TIMEOUT,
            StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            StatusCode::VARIANT_ALSO_NEGOTIATES,
            StatusCode::INSUFFICIENT_STORAGE,
            StatusCode::LOOP_DETECTED,
            StatusCode::NOT_EXTENDED,
            StatusCode::NETWORK_AUTHENTICATION_REQUIRED
        }
    }
}
