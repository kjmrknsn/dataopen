use chrono::prelude::*;
use iron::{AroundMiddleware, Handler};
use iron::prelude::*;
use super::super::log;

pub struct AccessLogger;

impl AroundMiddleware for AccessLogger {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(AccessLoggerHandler::new(handler))
    }
}

struct AccessLoggerHandler<T: Handler> {
    handler: T,
}

impl<T: Handler> AccessLoggerHandler<T> {
    fn new(handler: T) -> Self {
        AccessLoggerHandler {
            handler,
        }
    }
}

impl<T: Handler> Handler for AccessLoggerHandler<T> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let start = Local::now();
        let res = self.handler.handle(req);
        let end = Local::now();

        log::access(AccessLog::new(start, end, &req, &res).to_string().as_str());

        res
    }
}

struct AccessLog<T>
    where T: TimeZone {
    start: DateTime<T>,
    end: DateTime<T>,
    method: String,
    path: String,
    status: u16,
    error: String,
}

impl<T> AccessLog<T>
    where T: TimeZone {
    fn new(start: DateTime<T>, end: DateTime<T>, req: &Request, res: &IronResult<Response>) -> Self {
        let (res, error) = match *res {
            Ok(ref res) => (res, String::new()),
            Err(ref err) => (&err.response, format!("{}", err)),
        };

        let status = match res.status {
            Some(status) => status.to_u16(),
            None => 0,
        };

        AccessLog {
            start,
            end,
            method: req.method.to_string(),
            path: String::from("/") + req.url.path().join("/").as_str(),
            status,
            error,
        }
    }
}

impl<T> ToString for AccessLog<T>
    where T: TimeZone {
    fn to_string(&self) -> String {
        format!(
            "start:{:?}\t\
            end:{:?}\t\
            elapsed:{:?}\t\
            method:{}\t\
            path:{}\t\
            status:{}\t\
            error:{}",
            self.start,
            self.end,
            elapsed_milli(&self.start, &self.end),
            self.method,
            self.path,
            self.status,
            log::escape_tab(&self.error),
        )
    }
}

fn elapsed_milli<T>(start: &DateTime<T>, end: &DateTime<T>) -> f64
    where T: TimeZone {
    match end.time().signed_duration_since(start.time()).num_nanoseconds() {
        Some(elapsed) => elapsed as f64 / 1000000.0,
        None => -1.0,
    }
}
