use warp::Filter;

mod config;
mod io;
mod webpage_generator;

#[tokio::main]
async fn main() {
    let app_config = config::try_get_app_config().expect("Could not find app configuration");

    let api = warp::path("api").and(
        warp::path("coffee")
            .map(webpage_generator::generate_webpage)
            .or(warp::any().map(|| warp::http::StatusCode::NOT_FOUND)),
    ); // TODO refactor into api calls

    let static_files = warp::fs::dir("./www/static");

    let all_routes = api.or(static_files);

    eprintln!("Listening on {}", app_config.listen_on);

    warp::serve(all_routes).run(app_config.listen_on).await;
}
