use router::Router;
use super::handler;

pub fn new() -> Router {
    let mut router = Router::new();

    router.post(
        "/web/notebooks",
        handler::notebook::post,
        "post_notebook"
    );

    router.get(
        "/web/notebooks/:notebook_id/notebook_histories/:id",
        handler::notebook_history::get,
        "get_notebook_history"
    );

    router.post(
        "/web/notebooks/:notebook_id/notebook_histories",
        handler::notebook_history::post,
        "post_notebook_history"
    );

    router.patch(
        "/web/notebooks/:notebook_id/notebook_histories/:id/title",
        handler::notebook_history::patch_title,
        "patch_notebook_history_title"
    );

    router.get(
        "/web/uid",
        handler::uid::get,
        "get_uid"
    );

    router
}
