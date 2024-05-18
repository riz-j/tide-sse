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
        sender.send("message", "banana", None).await?;

        let app_state = req.state().clone();
        {
            let mut clients = app_state.clients.lock().await;
            clients.push(sender);
        }

        // Send messages to all clients when received
        loop {
            let message = req.state().receiver.recv().await.unwrap();
            let clients = req.state().clients.lock().await;
            for client in clients.iter() {
                if let Err(err) = client.send("message", &message, None).await {
                    println!("Error sending message: {}", err);
                }
            }
            println!("Received message: {}", message);
        }
    }

    // app.at("/sse2").get(sse::endpoint(
    //     |req: Request<AppState>, sender: Sender| async move {
    //         // let app_state = req.state().clone();
    //         // {
    //         //     let mut clients = app_state.clients.lock().unwrap();
    //         //     clients.push(sender);
    //         // }
    //         sender.send("message", "banana", None).await?;

    //         let clients = req.state().clients.lock().await;
    //         while let Ok(message) = &req.state().receiver.as_ref().recv().await {
    //             for s in clients.iter() {
    //                 s.send("message", message, None).await?;
    //             }

    //             // sender.send("message", message, None).await?;
    //             println!("Received message: {}", message);
    //         }

    //         Ok(())
    //     },
    // ));

    // async_std::task::spawn(async move {
    //     while let Ok(message) = receiver.recv().await {
    //         println!("Received: {}", message);
    //     }
    // });

    println!("App listening on port 8543");
    app.listen("127.0.0.1:8543").await?;
    Ok(())
}
