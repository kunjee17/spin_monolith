use crate::templates::get_tera;
use crate::types;
use spin_sdk::http::{IntoResponse, Params, Request, Response};
use uuid::Uuid;
use crate::types::Product;

fn get_products () -> Vec<Product> {
    let products = vec![
        Product {
            id: 1,
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: 100.0,
        },
        Product {
            id: 2,
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
            price: 200.0,
        },
        Product {
            id: 3,
            name: "Product 3".to_string(),
            description: "Description 3".to_string(),
            price: 300.0,
        },
    ];
    products
}

fn create_response_success(body: String) -> Response {
    Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(body)
        .build()
}

pub fn get_index(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let tera = get_tera()?;
    let context = tera::Context::new(); // This might be JSON as well
    Ok(create_response_success(tera.render("index.html", &context)?))
}

pub fn post_login(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let login_request: types::LoginRequest = serde_urlencoded::from_bytes(req.body())?;
    println!("Logging in user {:#?}", login_request);

    // Extract session ID from request headers or create a new one
    let session_id = if let Some((_, cookie)) = req.headers().find(|(name, _)| *name == "Cookie") {
        // Extract session ID from the cookie
        cookie
            .as_str()
            .map(|item| item.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string())
    } else {
        // Create a new unique session ID
        Uuid::new_v4().to_string()
    };
    println!("Session ID: {}", session_id);

    // Redirect to product page with session ID
    Ok(Response::builder()
        .status(302)
        .header("Location", "/product")
        .header(
            "Set-Cookie",
            format!("session_id={}; HttpOnly; Path=/", session_id),
        )
        .build())
}

pub fn get_register(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let tera = get_tera()?;
    let context = tera::Context::new(); // This might be JSON as well
    Ok(create_response_success(tera.render("register.html", &context)?))
}

pub fn post_register(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let register_request: types::RegisterRequest = serde_urlencoded::from_bytes(req.body())?;
    println!("Registering user {:#?}", register_request);
    if register_request.password != register_request.confirm_password {
        return Ok(Response::builder()
            .status(400)
            .body("Passwords do not match")
            .build());
    }
    // Redirect to login page
    Ok(Response::builder()
        .status(302)
        .header("Location", "/")
        .build())
}

pub fn get_product(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    req.header("Cookie").map_or_else(
        || Ok(Response::builder().status(401).body("Unauthorized").build()),
        |cookie| {
            println!("Cookie: {:#?}", cookie);
            let tera = get_tera()?;
            let mut context = tera::Context::new();
            context.insert("products", &get_products());
            Ok(create_response_success(tera.render("product.html", &context)?))
        },
    )
}
