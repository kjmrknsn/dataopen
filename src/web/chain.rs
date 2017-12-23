use iron::prelude::*;
use mysql::Pool;
use persistent;
use super::access_logger::AccessLogger;
use super::mysql_pool::MysqlPool;
use super::router;

pub fn new(mysql_pool: Pool) -> Chain {
    let mut chain = Chain::new(router::new());

    chain.link(persistent::Read::<MysqlPool>::both(mysql_pool));

    chain.around(AccessLogger);

    chain
}
