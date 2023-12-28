mod date;
mod web_server;

fn main() {
    if let Err(msg) = web_server::run() {
        eprintln!("Error: {}", msg);
    }
}
