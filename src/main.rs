use async_std::pin::Pin;
use chrono::Utc;
use std::future::Future;
use tide::{http::mime, utils::After, Next, Request, Response, Result, Server, StatusCode};

fn new_app() -> Server<()> {
    let mut app = tide::new();
    app
}

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = new_app();

    app.at("/").serve_file("static/index.html")?;
    app.at("/").serve_dir("static/")?;
    app.at("/generated").serve_dir("generated/")?;
    app.at("/images").serve_dir("images/")?;
    app.at("/articles").serve_dir("articles/")?;

    println!("Server running on port 8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
