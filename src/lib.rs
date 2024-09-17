mod templates;
mod api;
mod types;

use spin_sdk::http::{Request, Response, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_spin_monolith(req: Request) -> Response {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let mut router = Router::new();
    router.get("/", api::get_index);
    router.post("/login", api::post_login);
    router.get("/register", api::get_register);
    router.post("/register", api::post_register);
    router.get("/product", api::get_product);
    router.handle(req)
}
