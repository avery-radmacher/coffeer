mod date;
mod secret_manager;

pub fn generate_webpage() -> String {
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
