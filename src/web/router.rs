use router::Router;
use super::handler;

pub fn new() -> Router {
    let mut router = Router::new();

    router.post(
        "/web/notebooks",
        handler::notebook::post,
        "post_notebook"
    );

    router.post(
        "/web/notebooks/:notebook_id/notebook_histories",
        handler::notebook_history::post,
        "post_notebook_history"
    );

    router.get(
        "/web/uid",
        handler::uid::get,
        "get_uid"
    );

    router
}
