use actix_web::{web, App, HttpServer};
use log::{/*error, warn,*/ info, /*debug, trace, log, Level*/};
use std::thread;

use metric_videos::resources::{pages,api};
use metric_videos::settings::SETTINGS;
use metric_videos::updater;

/**
Main entry point.

This starts the ongoing threads for the cache updater and the HTTP listener.
Note that before execution even gets here, the configuration and logger have already been set up by
the lazy_static code in the settings module.

# Returns
Result, but only when actix-web fails to bind to the port we want to use for HTTP.
*/
#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    info!("Starting metric_videos on {}", &SETTINGS.startup.listen_addr);

    //Keep the DB updated while the app runs
    thread::spawn(|| { updater::updater(); });

    //Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/",                        web::get().to(pages::index))        // request for root: this delivers the main app page that users see
            .route("/dashboard",               web::get().to(pages::dashboard))    // dashboard page that shows the metrics
            .route("/api/metrics",             web::get().to(api::metrics))        // ajax calls to retrieve metrics
            .route("/api/watcher/{vid}/{pct}", web::post().to(api::watcher))       // ajax calls for watcher updates: we split part of the path into args
            .service(actix_files::Files::new("/static", "static").disable_content_disposition())   // serve static files from given dir
            .default_service(web::route().to(pages::notfound))                     // where to go when nothing else matches
    })
    .bind(&SETTINGS.startup.listen_addr)?
    .run()
    .await
}

