use crate::core::config::Config;
use std::env;

pub fn get_full_html_page(
    config: &Config,
    title: String,
    body_prefix: String,
    content: String,
    theme_variant: String,
    insert_base_element: bool,
) -> String {
    let base_url = match env::var("BASE_URL") {
        Ok(url) => url,
        Err(_) => "".to_string(),
    };
    let base_elem = {
        if insert_base_element {
            format!("<base href='{}/'>", base_url)
        } else {
            "".to_string()
        }
    };
    let favicons = config
        .app
        .favicons
        .iter()
        .map(|favicon| {
            format!(
                "<link rel=\"{rel}\" href=\"{base_url}{href}\">",
                rel = favicon.rel,
                base_url = base_url,
                href = favicon.href
            )
        })
        .collect::<Vec<String>>()
        .join("");
    format!(
        r"
        <!doctype html>
        <html>
            <head>
                <meta charset='utf-8' />
                <title>{title}</title>
                <script type='text/javascript' src='{base_elem}/viewy-static/js/importmap.js'></script>
                <link rel='preconnect' href='{base_url}'>
                {base_elem}
                {favicons}
                <link href='{base_url}/viewy-static/app.css' rel='stylesheet'>
                <link href='{base_url}/viewy-static/css/static.css' rel='stylesheet'>
                <script type='module' src='{base_url}/viewy-static/js/app.js'></script>
                <meta name='viewport' content='width=device-width, initial-scale=1.0, user-scalable=no'>
                <meta name='apple-mobile-web-app-capable' content='yes'>
            </head>
            <body class='app-theme--{theme_variant}'>
                {body_prefix}
                {content}
            </body>
        </html>
    ",
        title = title,
        body_prefix = body_prefix,
        content = content,
        theme_variant = theme_variant,
        base_elem = base_elem,
        favicons = favicons,
        base_url = base_url,
    )
}
