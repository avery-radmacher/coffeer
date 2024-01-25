pub use api::ApiConfig;
use warp::{filters::BoxedFilter, Filter, Reply};

mod api;

pub fn routes(config: api::ApiConfig) -> BoxedFilter<(impl Reply,)> {
    let static_files = warp::fs::dir("./www/static");
    api::api(config).or(static_files).boxed()
}
