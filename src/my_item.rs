use std::any::Any;

use tide::{Request, Response};

use crate::AppState;

trait MyResponse {
    fn new() -> Self;
    fn into_response(&self) -> tide::Result;
}

struct MyError {}
impl MyResponse for MyError {
    fn new() -> Self {
        MyError {}
    }

    fn into_response(&self) -> tide::Result {
        Ok(Response::builder(200)
            .body(r#"{ "message": "awesome stuff yo!" }"#)
            .header("Content-Type", "application/json")
            .header("Cache-Control", "no-cache")
            .build())
    }
}

pub async fn get_handler(req: Request<AppState>) -> tide::Result {
    let _ = &req.state().sender.send(String::from("Joe Schmoe")).await?;
    Ok(format!("Hello you!").into())
}

#[derive(serde::Deserialize)]
struct RequestBody {
    message: String,
}
pub async fn post_handler(mut req: Request<AppState>) -> tide::Result {
    let body = req.body_json::<RequestBody>().await.unwrap();
    let message = body.message.clone();
    // println!("{}", body.message.clone());

    let _ = req.state().sender.send(message).await?;

    Ok(format!("Message sent!").into())
}

pub async fn sse_spec(req: Request<AppState>) -> tide::Result {
    let clients = req.state().clients.lock().await;

    for client in clients.iter() {
        println!("{:?}", client.type_id());
    }

    MyError::new().into_response()
}
