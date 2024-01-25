use crate::webpage_generator;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn coffee() -> BoxedFilter<(impl Reply,)> {
    warp::path!("coffee")
        .map(webpage_generator::generate_webpage)
        .boxed()
}
