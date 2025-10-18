


use serde::{Deserialize, Serialize};
use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};
use mongodb::{ 
	bson::{doc},
	Client,
	Collection 
};
use futures::stream::TryStreamExt;



#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

#[test]
fn getmsg_test()  {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
	handler(Request::new(Body::Empty)).await.unwrap();
    }
    );
}
#[derive(Debug, Deserialize, Serialize)]
struct Message {
    sender: String,
    message: String
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    #[cfg(test)]
    let uri = "mongodb://localhost:27017";
    #[cfg(not(test))]
    let uri = std::env::var("MONGODB_URI")?;

    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;

    // Get a handle on the movies collection
    let database = client.database("opinion");
    let chats: Collection<Message> = database.collection("chat");
    let mut cursor = chats.find(doc!{}).await?;
    let mut ms: Vec<Message> = vec!{};
    while let Some(m) = cursor.try_next().await?  {
	ms.push(m);
    }
    //for m in messages.iter() {
    println!("message:\n{:#?}", ms);
    //}
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({"data": ms})
            .to_string()
            .into(),
        )?)
}
