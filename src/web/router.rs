use router::Router;
use super::handler;

pub fn new() -> Router {
    let mut router = Router::new();

    router.get("/web/uid", handler::uid::get, "get_uid");

    router
}
