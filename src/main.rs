use warp::Filter;

mod webpage_generator;

// fn main() {
//     if let Err(msg) = web_server::run() {
//         eprintln!("Error: {}", msg);
//     }
// }

#[tokio::main]
async fn main() {
    let coffee = warp::path!("coffee").map(webpage_generator::generate_webpage);

    warp::serve(coffee).run(([127, 0, 0, 1], 3030)).await;
}
