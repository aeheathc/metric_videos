pub mod api;
pub mod pages;

use crate::settings::SETTINGS;

fn page_header(menu: &str) -> String
{
    format!("<div><a href='/'>Home</a> <span>{}</span> <a href='/dashboard'>Dashboard</a></div>", menu)
}

/**
Generates a complete HTML document given the elements that change between pages.
This is where we define all the external static resources included in every page, and other HTML boilerplate.

# Parameters
- `title`: The contents of the title tag, which browsers tend to display in their title bar
- `head_extra`: HTML content to be included in the root of the head tag, intended for page-specific styles/scripts
- `body`: contents of the body tag

# Returns
String containing the HTML document.None
*/
fn html_construct(title: &str, head_extra: &str, body: &str) -> String
{
    format!("<!DOCTYPE html>
<html>
 <head>
  <meta charset='utf-8'/>
  <meta http-equiv='X-UA-Compatible' content='IE=edge'/>
  <meta name='viewport' content='height=device-height, width=device-width, initial-scale=1'/>
  <link rel='shortcut icon' href='static/favicon.ico'/>
  <script src='https://unpkg.com/jquery@3.5.1/dist/jquery.min.js'></script>
  <script src='https://unpkg.com/moment@2.19.3/min/moment-with-locales.min.js'></script>
  <link rel='stylesheet' href='static/main.css'/>
  <script>const videos = [\"{}\"];</script>
  {}
  <title>{}</title>
 </head>
 <body>
 {}
 </body>
</html>",
    &SETTINGS.media.videos.join("\",\""), head_extra, title, body)
}


/*
Test those functions which weren't able to have good tests as part of their
example usage in the docs, but are still possible to unit-test
*/
#[cfg(test)]
mod tests
{
    use super::*;

	// html_construct
	#[test]
	fn gen_page()
	{
        let html = html_construct("Not Found", "", "<h1>Not Found</h1><a href='/'>Return to Home</a>");
        assert_eq!(&html[..15],"<!DOCTYPE html>");
    }

}