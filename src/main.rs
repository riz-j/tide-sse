pub mod my_item;
pub mod my_sse;

use std::sync::Arc;

use async_std::channel;
use async_std::sync::Mutex;
use tide::sse;
use tide::sse::Sender;

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

    app.at("/").serve_file("public/index.html");

    app.at("/sse-test").get(sse::endpoint(my_sse::test_sse));

    app.at("/messages").post(post_handler);

    app.at("/sse").get(sse::endpoint(my_sse::sse_endpoint));

    app.at("/sse-spec").get(my_item::sse_spec);

    app.at("/*").serve_dir("public/");

    println!("App listening on port http://127.0.0.1:8543");
    app.listen("127.0.0.1:8543").await?;
    Ok(())
}
