mod date;
mod secret_manager;

pub fn generate_webpage() -> String {
    // get the secret
    let bet = secret_manager::get_or_create_bet();
    if !bet.date.is_future() {
        // if the secret is ready to be divulged, generate that webpage
        "(mock) HTML of divulged secret".to_owned()
    } else {
        // else, print a generic "not ready" webpage
        "(mock) \"Not Ready\" HTML".to_owned()
    }
}
