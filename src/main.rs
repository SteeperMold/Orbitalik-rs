use std::time::Duration;
use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files::Files;
use tera::Tera;

mod views;
mod forms;
mod calculations;
mod fetch_tle;

struct AppState {
    tmpl: Tera,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "warn");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    actix_rt::spawn(async {
        let fetching_settings = fetch_tle::read_settings("data/tle_fetching_settings.json");
        let mut interval = actix_rt::time::interval(Duration::from_secs(fetching_settings.delay_seconds));
        loop {
            interval.tick().await;
            match fetch_tle::fetch_tle(&fetching_settings).await {
                Ok(_) => log::warn!("Tle fethcing success"),
                Err(error) => log::warn!("Failed to fetch tle with this error: {:?}", error),
            };
        }
    });

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .wrap(Logger::default())
            .app_data(web::FormConfig::default().error_handler(forms::parse_error_handler))
            .app_data(web::Data::new(AppState { tmpl: tera }))
            .service(Files::new("/static", "static").show_files_listing())
            .route("/get-passes", web::get().to(views::get_passes))
            .route("/passes", web::get().to(views::show_passes))
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}
