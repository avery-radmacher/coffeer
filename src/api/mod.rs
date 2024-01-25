use warp::{filters::BoxedFilter, Filter, Reply};

mod coffee;

pub fn api() -> BoxedFilter<(impl Reply,)> {
    warp::path("api")
        .and(coffee::coffee().or(util::not_found_404()))
        .boxed()
}

mod util {
    use warp::{filters::BoxedFilter, Filter, Reply};

    pub fn not_found_404() -> BoxedFilter<(impl Reply,)> {
        warp::any()
            .map(|| warp::http::StatusCode::NOT_FOUND)
            .boxed()
    }
}
