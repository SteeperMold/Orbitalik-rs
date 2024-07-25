use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files::Files;
use tera::Tera;

mod views;
mod forms;
mod calculations;

struct AppState {
    tmpl: Tera,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "warn");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

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
