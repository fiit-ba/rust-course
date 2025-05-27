use std::sync::Arc;

use anyhow::Result;
use chat::Message;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::broadcast,
    task,
};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8000").await?;
    let (tx, _) = broadcast::channel(1024);
    let tx = Arc::new(tx);
    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let (tcp_read, tcp_write) = stream.into_split();
        println!("Connection established");

        task::spawn({
            let tx = tx.clone();
            async {
                handle_incoming(tcp_read, tx).await.ok();
            }
        });

        task::spawn({
            let rx = tx.subscribe();
            async {
                handle_outgoing(tcp_write, rx).await.ok();
            }
        });
    }
}

async fn handle_incoming(
    tcp_read: OwnedReadHalf,
    tx: impl AsRef<broadcast::Sender<Message>>,
) -> Result<()> {
    let mut tcp_read = BufReader::new(tcp_read).lines();
    let Some(inital_message) = tcp_read.next_line().await? else {
        return Ok(());
    };

    // Deserialize initial_message into a Message::User.
    // If the initial line is not a Message::User, stop this task.
    let user_message: Message = serde_json::from_str(&inital_message)?;
    let username = match user_message {
        Message::User(username) => {
            // Broadcast the user join message
            tx.as_ref().send(Message::User(username.clone())).ok();
            username
        }
        _ => {
            // If not a User message, stop this task
            return Ok(());
        }
    };

    // For each further incoming line, deserialize the line into a Message
    while let Some(line) = tcp_read.next_line().await? {
        let message: Message = match serde_json::from_str(&line) {
            Ok(msg) => msg,
            Err(_) => continue, // Skip invalid JSON
        };

        match message {
            // If the message is a Message::User, broadcast the message as-is using tx
            Message::User(_) => {
                tx.as_ref().send(message).ok();
            }
            // If the message is a Message::ClientMessage,
            // convert it into a Message::Chat and broadcast it using tx
            Message::ClientMessage(content) => {
                let chat_message = Message::Chat {
                    user: username.clone(),
                    content,
                };
                tx.as_ref().send(chat_message).ok();
            }
            // If the message is a Message::Chat, ignore it
            Message::Chat { .. } => {
                // Ignore chat messages from clients
            }
        }
    }

    Ok(())
}

async fn handle_outgoing(
    mut tcp_write: OwnedWriteHalf,
    rx: broadcast::Receiver<Message>,
) -> Result<()> {
    let mut rx = BroadcastStream::from(rx);
    while let Some(Ok(msg)) = rx.next().await {
        // Serialize message as JSON and send it to the client, along with a newline
        let json = serde_json::to_string(&msg)?;
        tcp_write.write_all(json.as_bytes()).await?;
        tcp_write.write_all(b"\n").await?;
        tcp_write.flush().await?;
    }
    Ok(())
}
