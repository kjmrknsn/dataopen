use iron::Iron;
use mysql::Pool;
use super::args::Args;
use super::chain;
use super::config::Config;
use super::super::log;

pub fn run() {
    let args = Args::new();

    if args.print_version {
        println!("{}", appname_version());
        return;
    }

    let conf = Config::from(&args.conf_path);

    let mysql_pool = Pool::new(&conf.mysql.url).unwrap();

    log::info(&appname_version());
    log::info(&format!(
        "Listening on {}.",
        conf.http.addr
    ));

    Iron::new(chain::new(mysql_pool))
        .http(&conf.http.addr).unwrap();
}

fn appname_version() -> String {
    format!("Data Open Web {}", env!("CARGO_PKG_VERSION"))
}
