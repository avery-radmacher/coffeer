use warp::Filter;

mod api;
mod config;
mod io;
mod webpage_generator;

#[tokio::main]
async fn main() {
    let app_config = config::try_get_app_config().expect("Could not find app configuration");

    let static_files = warp::fs::dir("./www/static");

    let all_routes = api::api().or(static_files);

    eprintln!("Listening on {}", app_config.listen_on);

    warp::serve(all_routes).run(app_config.listen_on).await;
}
