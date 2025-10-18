use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // let str = match _req.body() {
    //     Body::Text(str) => str,
    //     Body::Binary(bin) => &String::from_utf8(bin.clone()).unwrap(),
    //     _ => "",
    // };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "message": "Hello, world! This is a Rust Vercel server."
            })
            .to_string()
            .into(),
        )?)
}
