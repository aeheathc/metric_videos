use actix_web::{web, HttpRequest, HttpResponse, http::header, http::StatusCode};
use actix_http::ResponseBuilder;
use log::{/*error, */warn, /*info, debug, trace, log, Level*/};

use std::net::{IpAddr};

use crate::metrics::METRICS;


/**
Responds to requests for the api endpoint metrics

# Returns
HttpResponse containing (if successful) JSON with the requested data.
*/
pub async fn metrics() -> HttpResponse
{
    match METRICS.lock()
    {
        Ok(g)  => ResponseBuilder::new(StatusCode::OK                 ).set_header(header::CONTENT_TYPE, "application/json; charset=utf-8").json(&*g),
        Err(_) => ResponseBuilder::new(StatusCode::SERVICE_UNAVAILABLE).set_header(header::CONTENT_TYPE, "application/json; charset=utf-8").json(0)
    }
}

/**
Responds to requests for the api endpoint "watcher"

# Parameters
- `watcher_data`: actix-generated tuple containing the captured parameters "vid" (video id) and "pct" (percent completion)

# Returns
HttpResponse with a blank body.

# Errors
The HTTP status code can indicate failure, which happens when anything goes wrong like invalid input.
*/
pub async fn watcher(req: HttpRequest, watcher_data: web::Path<(usize, u8)>) -> HttpResponse
{
    let (video_index, percent) = (watcher_data.0, watcher_data.1);

    let ip: IpAddr = match req.peer_addr()
    {
        Some(a) => a.ip(),
        None => {
            warn!("Got API request without a source address, discarding");
            return ResponseBuilder::new(StatusCode::FAILED_DEPENDENCY).body("");
        }
    };

    match METRICS.try_lock()
    {
        Ok(mut g) => {
            g.report(ip, video_index, percent);
        },
        Err(_) => {return ResponseBuilder::new(StatusCode::SERVICE_UNAVAILABLE).body("");}
    };
    
    ResponseBuilder::new(StatusCode::OK)
        .body("")
}