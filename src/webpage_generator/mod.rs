mod date;
pub mod secret_manager;

pub fn generate_webpage(party_1: String, party_2: String) -> warp::reply::Json {
    warp::reply::json(&secret_manager::get_or_create_bet(party_1, party_2).divulge_if_mature())
}
