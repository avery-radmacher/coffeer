use crate::secret_manager;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn coffee(party_1: String, party_2: String) -> BoxedFilter<(impl Reply,)> {
    warp::path!("coffee")
        .map(move || {
            warp::reply::json(
                &secret_manager::get_or_create_bet(party_1.clone(), party_2.clone())
                    .divulge_if_mature(),
            )
        })
        .boxed()
}
