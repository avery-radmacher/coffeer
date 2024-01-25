use warp::{filters::BoxedFilter, Filter, Reply};

mod coffee;

pub struct ApiConfig {
    pub party_1: String,
    pub party_2: String,
}

pub fn api(config: ApiConfig) -> BoxedFilter<(impl Reply,)> {
    warp::path("api")
        .and(coffee::coffee(config.party_1, config.party_2).or(util::not_found_404()))
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
