use router::Router;
use super::handler;

pub fn new() -> Router {
    let mut router = Router::new();

    router.post(
        "/web/notebook",
        handler::notebook::post,
        "post_notebook"
    );
    router.get(
        "/web/uid",
        handler::uid::get,
        "get_uid"
    );

    router
}
