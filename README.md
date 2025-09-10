# Viewy

A web UI toolkit that combines the strengths of a design system with the ergonomics of a UI library. Build HTML pages from composable, strongly-typed Rust components and render either full pages (with head and assets) or just the content.

- Composable, declarative components (Text, Button, View, Forms, etc.)
- Flexible layouts (pass a closure to wrap your content)
- Configurable theme and assets
- Renders to plain HTML + CSS/JS
- Suitable for small apps and more complex projects

Status: beta (2.0.0-beta.4). Rust 1.87.0, edition 2024.

## Install

From crates.io:

```bash
cargo add viewy@=2.0.0-beta.4
```

Or add to Cargo.toml:

```toml
[dependencies]
viewy = "2.0.0-beta.4"
```

Docs: https://docs.rs/viewy
Repository: https://github.com/strowbeary/viewy-rs

## Quick start (with Rocket)

The example below shows how to render a simple page using a minimal layout that wraps your content inside a container View.

```rust
use rocket::{get, launch, routes};
use rocket::response::content::RawHtml;
use viewy::{Page, RenderMode};
use viewy::components::{Text, TextStyle, View};

#[get("/")]
fn index() -> RawHtml<String> {
    // Define a minimal layout: wrap content in a top-level View
    let layout: viewy::Layout = Box::new(|content| {
        let mut view = View::new();
        // push the page content into the container
        view.node.children.push(content);
        view.render()
    });

    let html = Page::new(
        "Hello World – Viewy",
        layout,
        Text::new("Hello, world!", TextStyle::LargeTitle),
    )
    .compile(RenderMode::Complete);

    RawHtml(html)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index])
}
```

## Basic concepts

- Components: Everything that can render implements Renderable and produces a Node. Common components live under viewy::components.
- Layout: A Layout is a Box<dyn Fn(Node) -> Node>. It receives the page content Node and returns the final Node tree (e.g., a shell with navigation, footer, etc.).
- Page: Page::new(name, layout, content).compile(RenderMode::Complete | ContentOnly | LayoutOnly) renders HTML.

## Rendering modes

- Complete: full HTML page with head, assets, and body.
- ContentOnly: only the content Node as HTML (embed into an existing page or template).
- LayoutOnly: renders the layout shell with a placeholder for content.

## Theming and assets

Viewy can inject styles and scripts into the generated page and supports light/dark themes. You can control global settings (favicons, base URL, etc.) via configuration loaded by viewy::Config. See the crate docs for details.

## Workspace

This repository is a Cargo workspace:
- lib: the viewy crate (library)
- codegen: build-time helpers for assets/styles
- site: example site and assets

## License

MIT © Rémi Caillot
