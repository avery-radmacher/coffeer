use warp::Filter;

mod webpage_generator;

#[tokio::main]
async fn main() {
    let api = warp::path("api")
        .and(warp::path("coffee"))
        .map(webpage_generator::generate_webpage); // TODO refactor into api calls
                                                   // TODO handle failures instead of falling back to looking up /api in the filesystem

    let static_files = warp::fs::dir("./www/static"); // TODO assume requests without file extensions are html requests

    let all_routes = api.or(static_files);

    warp::serve(all_routes).run(([127, 0, 0, 1], 3030)).await;
}
