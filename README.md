# Viewy

A UI toolkit that combine the advantages of a design system and an ui library.

- Customizable theme via config file (planned)
- For small and more complex projects

# "Hello, World!" example program (rocket.rs)

```rust
use viewy::*;
use viewy::components::*;
#[macro_use] extern crate rocket;

pub fn default_layout() -> Box<dyn Fn(Box<dyn Renderable>) -> Box<dyn Renderable>> {
    Box::new(move |content| Box::new({
        content
    }))
}

#[get("/")]
fn index() -> Html<String> {
   Html({
       Page::new(
           &format!("{} – Viewy", "Hello World"),
           &default_layout(),
           {
               Text::new("Hello world", TextStyle::LargeTitle)
           }
       )
           .compile(RenderMode::Complete)
   })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

```
