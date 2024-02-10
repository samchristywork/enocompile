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

const NOT_FOUND_HTML_PAGE: &str = "<html><body>
  <h1>File Not Found</h1>
  <p>Error 404: The requested file was not found on this server.</p>
</body></html>";

const INTERNAL_SERVER_ERROR_HTML_PAGE: &str = "<html><body>
  <h1>Internal Server Error</h1>
  <p>Error 500: The server encountered an internal error and was unable to complete your request.</p>
</body></html>";

fn new_app() -> Server<()> {
    let mut app = tide::new();

    app.with(After(|response: Response| async move {
        Ok(match response.status() {
            StatusCode::InternalServerError => Response::builder(500)
                .content_type(mime::HTML)
                .body(INTERNAL_SERVER_ERROR_HTML_PAGE)
                .build(),

            _ => response,
        })
    }));

    app.with(After(|response: Response| async move {
        Ok(match response.status() {
            StatusCode::NotFound => Response::builder(404)
                .content_type(mime::HTML)
                .body(NOT_FOUND_HTML_PAGE)
                .build(),
            _ => response,
        })
    }));

    app.with(log_middleware);

    app
}

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = new_app();

    app.at("/").serve_file("generated/index.html")?;
    app.at("/").serve_dir("generated/")?;
    app.at("/images").serve_dir("images/")?;
    app.at("/articles").serve_dir("articles/")?;

    println!("Server running on port 8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
