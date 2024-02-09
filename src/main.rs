use async_std::pin::Pin;
use chrono::Utc;
use std::future::Future;
use tide::{http::mime, utils::After, Next, Request, Response, Result, Server, StatusCode};

fn log_middleware<'a>(
    req: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async move {
        let method = req.method();
        let route = req.url().path().to_string();
        let params = req.url().query().unwrap_or("").to_string();
        let res = next.run(req).await;
        let status = res.status();
        let date = Utc::now().format("%Y-%m-%d %H:%M:%S");

        match status.is_success() {
            true => {
                println!(
                    "{} {} \x1b[32m{}\x1b[0m {} \x1b[90m[{}]\x1b[0m",
                    date, method, status, route, params
                );
            }
            false => {
                println!(
                    "{} {} \x1b[31m{}\x1b[0m {} \x1b[90m[{}]\x1b[0m",
                    date, method, status, route, params
                );
            }
        }
        Ok(res)
    })
}

fn new_app() -> Server<()> {
    let mut app = tide::new();

    app.with(log_middleware);

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
