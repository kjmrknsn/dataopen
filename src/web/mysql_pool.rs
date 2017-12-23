use iron::typemap::Key;
use mysql::Pool;

pub struct MysqlPool;

impl Key for MysqlPool {
    type Value = Pool;
}
