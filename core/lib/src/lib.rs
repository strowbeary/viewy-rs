extern crate minifier;
extern crate grass;
extern crate serde;
extern crate serde_json;
extern crate html_escape;
extern crate uuid;

mod helper_fn;
mod node;
mod modifiers;
mod renderer;
pub mod components;

pub use modifiers::DefaultModifiers;
pub use renderer::ToHtml;
pub use helper_fn::*;

pub fn compile_page((content, style, script): (String, String, String), theme_variant: &str) -> String {
    format!(r"
        <!doctype html>
        <html>
        <head>
            <title>Viewy-rs showcase</title>
            <style>{style}</style>
            <script>{script}</script>
            <script src='https://unpkg.com/@popperjs/core@2'></script>
            <meta charset='utf8' />
        </head>
        <body class='app-theme--{theme_variant}'>
            {content}
        </body>
        </html>
    ",
            style = style,
            script = script,
            content = content,
            theme_variant = theme_variant
    )
}

