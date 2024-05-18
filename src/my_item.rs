use tide::Request;

use crate::AppState;

pub async fn get_handler(req: Request<AppState>) -> tide::Result {
    let _ = &req.state().sender.send(String::from("Joe Schmoe")).await?;
    Ok(format!("Hello you!").into())
}

#[derive(serde::Deserialize)]
struct RequestBody {
    message: String,
}
pub async fn post_handler(mut req: Request<AppState>) -> tide::Result {
    let body: &RequestBody = &req.body_json().await.unwrap();
    let message = body.message.clone();
    // println!("{}", body.message.clone());

    let _ = &req.state().sender.send(message).await?;

    Ok(format!("Message sent!").into())
}

pub async fn sse_spec(req: Request<AppState>) -> tide::Result {
    let clients = req.state().clients.lock().await;

    for client in clients.iter() {
        println!("{:?}", client.type_id());
    }

    Ok(format!("Check your terminal").into())
}
