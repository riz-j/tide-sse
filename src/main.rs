pub mod my_item;
pub mod my_sse;

use std::sync::Arc;

use async_std::channel;
use async_std::sync::Mutex;
use tide::sse;
use tide::sse::Sender;
use tide::Request;

use crate::my_item::post_handler;

#[derive(Clone)]
pub struct AppState {
    pub name: String,
    pub sender: Arc<channel::Sender<String>>,
    pub receiver: Arc<channel::Receiver<String>>,
    pub clients: Arc<Mutex<Vec<Sender>>>,
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
            clients: Arc::new(Mutex::new(Vec::new())),
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

    app.at("/sse").get(sse::endpoint(sse_endpoint));
    async fn sse_endpoint(req: Request<AppState>, sender: Sender) -> tide::Result<()> {
        {
            sender.send("message", "Connected!", None).await?;
            // When client connects, append the sender to the AppState's client vector
            let mut clients = req.state().clients.lock().await;
            clients.push(sender);
            println!("A client just connected!");
        }

        // Send messages to all clients when received
        loop {
            let message = req.state().receiver.recv().await.unwrap();
            {
                let clients = req.state().clients.lock().await;
                println!("{:#?}", clients);

                for client in clients.iter() {
                    let _ = client.send("message", &message, None).await;
                }
            }

            println!("Received message: {}", message);
        }
    }

    println!("App listening on port 8543");
    app.listen("127.0.0.1:8543").await?;
    Ok(())
}
