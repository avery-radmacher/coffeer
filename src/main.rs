use warp::Filter;

mod webpage_generator;

#[tokio::main]
async fn main() {
    let api = warp::path("api").and(
        warp::path("coffee")
            .map(webpage_generator::generate_webpage)
            .or(warp::any().map(|| warp::http::StatusCode::NOT_FOUND)),
    ); // TODO refactor into api calls

    let static_files = warp::fs::dir("./www/static"); // TODO assume requests without file extensions are html requests

    let all_routes = api.or(static_files);

    warp::serve(all_routes).run(([127, 0, 0, 1], 3030)).await;
}
