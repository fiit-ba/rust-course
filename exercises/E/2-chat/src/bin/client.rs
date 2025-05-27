use anyhow::Result;
use chat::Message;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, Stdin},
    join,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    task,
};

#[tokio::main]
async fn main() -> Result<()> {
    let stdin = tokio::io::stdin();
    let mut stdin_lines = BufReader::new(stdin).lines();

    println!("Enter your username and press <enter>");
    let username = stdin_lines.next_line().await?.unwrap();
    let username = Message::User(username);
    println!("Connecting to server...");
    let stream = TcpStream::connect("127.0.0.1:8000").await?;
    let (tcp_read, mut tcp_write) = stream.into_split();

    // Send username to the server as JSON, along with a newline
    let json = serde_json::to_string(&username)?;
    tcp_write.write_all(json.as_bytes()).await?;
    tcp_write.write_all(b"\n").await?;
    tcp_write.flush().await?;

    println!("Connected! You can now enter messages!");

    let chat_input_task = task::spawn(handle_chat_input(stdin_lines, tcp_write));
    let incoming_chats_task = task::spawn(handle_incoming_chats(tcp_read));
    let _ = join!(chat_input_task, incoming_chats_task);
    Ok(())
}

async fn handle_chat_input(
    mut stdin: Lines<BufReader<Stdin>>,
    mut tcp_write: OwnedWriteHalf,
) -> Result<()> {
    // For every line of stdin, create a Message::ClientMessage
    // containing the line as content, and send it to the server,
    // along with a newline
    while let Some(line) = stdin.next_line().await? {
        let message = Message::ClientMessage(line);
        let json = serde_json::to_string(&message)?;
        tcp_write.write_all(json.as_bytes()).await?;
        tcp_write.write_all(b"\n").await?;
        tcp_write.flush().await?;
    }
    Ok(())
}

async fn handle_incoming_chats(tcp_read: OwnedReadHalf) -> Result<()> {
    let mut tcp_read = BufReader::new(tcp_read).lines();
    while let Ok(Some(message)) = tcp_read.next_line().await {
        match serde_json::from_str(&message)? {
            Message::Chat { content, user } => {
                println!("<{user}>: {content}")
            }
            Message::User(username) => {
                println!("<{username}> joined the chat")
            }
            _ => {} // Let's just ignore these
        }
    }

    Ok(())
}
