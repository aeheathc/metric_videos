use std::path::Path;

use actix_web::{HttpResponse, http::header, http::StatusCode};
use actix_http::ResponseBuilder;
/*use log::{error, warn, info, debug, trace, log, Level};*/

use crate::settings::SETTINGS;
use super::html_construct;
use super::page_header;

/**
Responds to requests for the main page at the domain root.

# Returns
HttpResponse containing the main page
*/
pub async fn index() -> HttpResponse
{
    let mut video_links = String::new();
    for index in 0..SETTINGS.media.videos.len()
    {
        let vid_name = match Path::new(&SETTINGS.media.videos[index]).file_name()
        {
            Some(s) => s.to_string_lossy().into_owned(),
            None => format!("Video {}",index)
        };
        let link = format!("<a onclick='switchVid({})'>{}</a>", index, vid_name);
        video_links.push_str(&link);
    }
    
    let body = format!("{}<video id='player' controls disablePictureInPicture />", page_header(&video_links));
    let head = "<script src='static/video.js'></script>";

    let html = html_construct("Home - Metric Videos", head, &body);

    ResponseBuilder::new(StatusCode::OK)
        .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(html)
}

/**
Responds to requests for the metrics dashboard.

# Returns
HttpResponse containing the dashboard page
*/
pub async fn dashboard() -> HttpResponse
{
    let body = format!("{}<div id='dashboard'></div><script src='/static/startReact.js'></script>", page_header(""));
    let head = "
    <script src='https://unpkg.com/react@16/umd/react.production.min.js'></script>
    <script src='https://unpkg.com/react-dom@16/umd/react-dom.production.min.js'></script>
    <!--<script crossorigin src='https://unpkg.com/react@16/umd/react.development.js'></script>
    <script crossorigin src='https://unpkg.com/react-dom@16/umd/react-dom.development.js'></script>-->
    <script src='static/dashboard.js'></script>";

    let html = html_construct("Home - Metric Videos", head, &body);

    ResponseBuilder::new(StatusCode::OK)
        .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(html)
}

/**
Responds to requests that don't match anything we have.

# Returns
HttpResponse indicating HTTP 404 Not Found.
*/
pub async fn notfound() -> HttpResponse
{
    let html = html_construct("Not Found - Metric Videos", "", "<h1>Not Found</h1><a href='/'>Return to Home</a>");

    ResponseBuilder::new(StatusCode::NOT_FOUND)
        .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(html)
}