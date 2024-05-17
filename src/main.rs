pub mod my_item;
pub mod my_sse;

use std::sync::Arc;

use async_std::channel;
use async_std::task;

#[derive(Clone)]
pub struct AppState {
    pub name: String,
    pub sender: Arc<channel::Sender<u32>>,
    pub receiver: Arc<channel::Receiver<u32>>,
}
impl AppState {
    fn new(
        name: String,
        sender: Arc<channel::Sender<u32>>,
        receiver: Arc<channel::Receiver<u32>>,
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
    let channel = channel::bounded::<u32>(100);
    let sender = Arc::new(channel.0);
    let receiver = Arc::new(channel.1);

    let app_state = AppState::new(
        String::from("my-mpsc-app"),
        sender.clone(),
        receiver.clone(),
    );

    let mut app = tide::with_state(app_state);

    app.at("/").get(my_item::get_handler);

    task::spawn(async move {
        while let Ok(message) = receiver.recv().await {
            println!("Received: {}", message);
        }
    });

    println!("App listening on port 8543");
    app.listen("127.0.0.1:8543").await?;
    Ok(())
}
