mod config;
mod io;
mod routes;
mod webpage_generator;

#[tokio::main]
async fn main() {
    let app_config = config::try_get_app_config().expect("Could not find app configuration");

    let all_routes = routes::routes();

    eprintln!("Listening on {}", app_config.listen_on);

    warp::serve(all_routes).run(app_config.listen_on).await;
}
