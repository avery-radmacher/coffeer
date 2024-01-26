use warp::{filters::BoxedFilter, Filter, Reply};

mod coffee;

pub struct ApiConfig {
    pub party_1: String,
    pub party_2: String,
}

pub fn api(config: ApiConfig) -> BoxedFilter<(impl Reply,)> {
    warp::path("api")
        .and(
            coffee::coffee(config.party_1, config.party_2)
                .or(balance::balance())
                .or(util::not_found_404()),
        )
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

mod balance {
    use crate::io;
    use std::path::PathBuf;
    use warp::{filters::BoxedFilter, Filter, Reply};

    fn get_path() -> PathBuf {
        io::storage_directory().join("balance.json")
    }

    fn get_balance() -> i32 {
        let balance = io::try_read_json_file(&get_path());
        if let Some(balance) = balance {
            return balance;
        }
        set_balance(0);
        return 0;
    }

    fn set_balance(balance: i32) {
        io::write_json_file(&get_path(), &balance);
    }

    fn get() -> BoxedFilter<(impl Reply,)> {
        warp::get()
            .map(|| warp::reply::json(&get_balance()))
            .boxed()
    }

    fn put() -> BoxedFilter<(impl Reply,)> {
        // TODO auth
        warp::put()
            .and(warp::body::json())
            .map(|balance: i32| {
                set_balance(balance);
                warp::reply()
            })
            .boxed()
    }

    pub fn balance() -> BoxedFilter<(impl Reply,)> {
        warp::path!("balance").and(get().or(put())).boxed()
    }
}
