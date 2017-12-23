use mysql::Pool;
use super::config::Mysql;

pub struct MysqlPool {
    pub pool: Pool,
}

impl MysqlPool {
    pub fn new(conf: &Mysql) -> Self {
        let pool = Pool::new(&conf.url).unwrap();

        MysqlPool {
            pool,
        }
    }
}


