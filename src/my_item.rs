use tide::Request;

use crate::AppState;

pub async fn get_handler(req: Request<AppState>) -> tide::Result {
    let _ = &req.state().sender.send(69).await?;
    Ok(format!("Hello you!").into())
}
