use crate::webpage_generator;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn coffee(party_1: String, party_2: String) -> BoxedFilter<(impl Reply,)> {
    warp::path!("coffee")
        .map(move || webpage_generator::generate_webpage(party_1.clone(), party_2.clone()))
        .boxed()
}
