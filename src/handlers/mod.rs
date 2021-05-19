use crate::middlewares::{auth::auth_handler, logger::*};
use salvo::extra::serve::*;
use salvo::prelude::*;

mod users;
use users::*;

#[fn_handler]
async fn index(res: &mut Response) {
    res.set_status_code(StatusCode::OK);
    res.render_plain_text(include_str!("../../readme.md"));
}

pub fn router() -> Router {
    let op = Options {
        dot_files: true,
        listing: true,
        defaults: vec![],
    };

    let user = Router::new()
        .path(r#"user"#)
        .push(Router::new().path(r#"register"#).post(register))
        .push(Router::new().path(r#"login"#).post(login))
        .push(
            Router::new()
                .before(auth_handler)
                .path(r#"info"#)
                .get(user_info),
        );

    let api = Router::new().path("api").push(user);

    Router::new()
        .before(logger_initializer)
        .after(logger_printer)
        .after(salvo::extra::compression::gzip())
        .get(index)
        .push(
            Router::new()
                .path(r#"assets/<**path>"#)
                .get(StaticDir::width_options(vec!["assets"], op)),
        )
        .push(api)
}
