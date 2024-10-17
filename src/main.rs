use std::time::Duration;
use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;

mod views;
mod forms;
mod calculations;
mod fetch_tle;
mod serializers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    std::env::set_var("TLE_FETCHING_SETTINGS_PATH", "data/tle_fetching_settings.json");
    std::env::set_var("TLE_FILE_PATH", "data/tle.txt");

    actix_rt::spawn(async {
        let mut fetching_settings = fetch_tle::read_settings().await;
        let mut interval = actix_rt::time::interval(Duration::from_secs(fetching_settings.delay_seconds));
        loop {
            interval.tick().await;
            match fetch_tle::fetch_tle(&mut fetching_settings).await {
                Ok(_) => log::info!("Tle fethcing success"),
                Err(error) => log::warn!("Failed to fetch tle with this error: {:?}", error),
            };
        }
    });

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())  // TODO Удалить
            .app_data(web::FormConfig::default().error_handler(forms::parse_error_handler))
            .route("api/get-satellites-list", web::get().to(views::get_satellites_list))
            .route("api/get-satellite-data", web::get().to(views::get_satellite_data))
            .route("api/get-passes-list", web::get().to(views::get_passes_list))
            .route("api/get-trajectory", web::get().to(views::get_trajectory))
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}
