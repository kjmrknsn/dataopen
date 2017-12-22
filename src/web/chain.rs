use iron::prelude::*;
use super::access_logger::AccessLogger;
use super::router;

pub fn new() -> Chain {
    let mut chain = Chain::new(router::new());

    chain.around(AccessLogger);

    chain
}
