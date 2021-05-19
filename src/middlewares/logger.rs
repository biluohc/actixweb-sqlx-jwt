use salvo::prelude::*;
use std::time::*;

static REQ_ARRIVED: &str = "reqArrived";

#[fn_handler]
pub async fn logger_initializer(depot: &mut Depot) {
    depot.insert(REQ_ARRIVED, Instant::now())
}

#[fn_handler]
pub async fn logger_printer(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let starts: &Instant = depot.borrow(REQ_ARRIVED);
    let cost = starts.elapsed();

    let addr = req.remote_addr().unwrap();
    let method = req.method();
    let uri = req.uri();

    if let Some(c) = res.status_code() {
        info!(
            target: "mwlogger",
            "{} {} \"{}\" {} '{}' {:?}",
            addr,
            method,
            uri,
            c.as_u16(),
            c.canonical_reason().unwrap_or("Null"),
            cost
        )
    } else {
        error!(target: "mwlogger", "{} {} \"{}\" 'statusCode is None' {:?}", addr, method, uri, cost)
    }
}
