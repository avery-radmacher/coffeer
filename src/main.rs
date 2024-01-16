use warp::Filter;

mod webpage_generator;

#[tokio::main]
async fn main() {
    let coffee = warp::path!("coffee").map(webpage_generator::generate_webpage); // TODO refactor into api calls

    let static_files = warp::fs::dir("./www/static"); // TODO assume requests without file extensions are html requests

    let all_routes = static_files.or(coffee);

    warp::serve(all_routes).run(([127, 0, 0, 1], 3030)).await;
}
