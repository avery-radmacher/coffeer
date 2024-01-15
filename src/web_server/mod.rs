pub mod webpage_generator;

pub fn run() -> Result<(), String> {
    let mut request_satisfied = false;
    while !request_satisfied {
        let page = webpage_generator::generate_webpage();
        println!("{}", page);
        request_satisfied = true;
    }
    Err("Server is shutting down".to_owned())
}
