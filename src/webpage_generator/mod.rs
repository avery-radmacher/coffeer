mod date;
pub mod secret_manager;

pub fn generate_webpage_text() -> String {
    // get the secret
    let bet = secret_manager::get_or_create_bet();
    if !bet.date.is_future() {
        // if the secret is ready to be divulged, generate that webpage
        bet.reveal()
    } else {
        // else, print a generic "not ready" webpage
        "The bet has not yet reached maturity.".to_owned()
    }
}

pub fn generate_webpage() -> warp::reply::Json {
    // get the secret
    let bet = secret_manager::get_or_create_bet();
    let bet = if !bet.date.is_future() {
        // if the secret is ready to be divulged, generate that webpage
        Some(bet)
    } else {
        // else, print a generic "not ready" webpage
        None
    };

    warp::reply::json(&bet)
}
