pub mod my_item;
pub mod my_sse;

use std::sync::Arc;

use async_std::channel;
use tide::sse;
use tide::Request;

use crate::my_item::post_handler;

#[derive(Clone)]
pub struct AppState {
    pub name: String,
    pub sender: Arc<channel::Sender<String>>,
    pub receiver: Arc<channel::Receiver<String>>,
}
impl AppState {
    fn new(
        name: String,
        sender: Arc<channel::Sender<String>>,
        receiver: Arc<channel::Receiver<String>>,
    ) -> Self {
        AppState {
            name,
            sender,
            receiver,
        }
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let channel = channel::bounded::<String>(100);
    let sender = Arc::new(channel.0);
    let receiver = Arc::new(channel.1);

    let app_state = AppState::new(
        String::from("my-mpsc-app"),
        sender.clone(),
        receiver.clone(),
    );

    let mut app = tide::with_state(app_state);

    app.at("/").get(my_item::get_handler);

    app.at("/sse-test")
        .get(sse::endpoint(|_req, sender| async move {
            sender.send("fruit", "banana", None).await?;
            loop {
                sender.send("fruit", "apple", None).await?;
                async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            }
        }));

    app.at("/messages").post(post_handler);

    app.at("/sse")
        .get(sse::endpoint(|req: Request<AppState>, sender| async move {
            sender.send("message", "banana", None).await?;

            while let Ok(message) = &req.state().receiver.as_ref().recv().await {
                sender.send("message", message, None).await?;
                println!("Received message: {}", message);
            }

            Ok(())
        }));

    // async_std::task::spawn(async move {
    //     while let Ok(message) = receiver.recv().await {
    //         println!("Received: {}", message);
    //     }
    // });

    println!("App listening on port 8543");
    app.listen("127.0.0.1:8543").await?;
    Ok(())
}
