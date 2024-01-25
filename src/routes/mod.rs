use warp::{filters::BoxedFilter, Filter, Reply};

mod api;

pub fn routes() -> BoxedFilter<(impl Reply,)> {
    let static_files = warp::fs::dir("./www/static");
    api::api().or(static_files).boxed()
}
