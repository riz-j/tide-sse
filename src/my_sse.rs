use tide::sse::Sender;
use tide::Request;

use crate::AppState;

pub async fn sse_endpoint(req: Request<AppState>, sender: Sender) -> tide::Result<()> {
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
        let clients = req.state().clients.lock().await;

        let send_futures: Vec<_> = clients
            .iter()
            .map(|client| client.send("message", &message, None))
            .collect();

        let _ = futures::future::join_all(send_futures).await;

        println!("Received message: {}", message);
    }
}

// async_std::task::spawn(async move {
//     while let Ok(message) = receiver.recv().await {
//         println!("Received: {}", message);
//     }
// });

pub async fn test_sse(_req: Request<AppState>, sender: Sender) -> tide::Result<()> {
    sender.send("fruit", "banana", None).await?;
    loop {
        sender.send("fruit", "apple", None).await?;
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    }
}
