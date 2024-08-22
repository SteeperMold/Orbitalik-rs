use std::time::Duration;
use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use actix_files::Files;
use tera::Tera;

mod views;
mod forms;
mod calculations;
mod fetch_tle;
mod serializers;

struct AppState {
    tera: Tera,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    std::env::set_var("TLE_FETCHING_SETTINGS_PATH", "data/tle_fetching_settings.json");
    std::env::set_var("TLE_FILE_PATH", "data/tle.txt");

    actix_rt::spawn(async {
        let path = std::env::var("TLE_FETCHING_SETTINGS_PATH").unwrap();
        let fetching_settings = fetch_tle::read_settings(&path);
        let mut interval = actix_rt::time::interval(Duration::from_secs(fetching_settings.delay_seconds));
        loop {
            interval.tick().await;
            match fetch_tle::fetch_tle(&fetching_settings).await {
                Ok(_) => log::info!("Tle fethcing success"),
                Err(error) => log::warn!("Failed to fetch tle with this error: {:?}", error),
            };
        }
    });

    let app_state = web::Data::new(AppState {
        tera: Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())  // TODO Удалить
            .app_data(web::FormConfig::default().error_handler(forms::parse_error_handler))
            .app_data(app_state.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("api/get-satellites-list", web::get().to(views::get_satellites_list))
            .route("api/get-satellite-data", web::get().to(views::get_satellite_data))
            .route("/get-passes", web::get().to(views::get_passes))
            .route("/passes", web::get().to(views::show_passes))
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}
