mod date;
pub mod secret_manager;

pub fn generate_webpage() -> warp::reply::Json {
    warp::reply::json(&secret_manager::get_or_create_bet().divulge_if_mature())
}
