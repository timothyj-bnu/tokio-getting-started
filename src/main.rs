use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind listened
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // second item is IP and port
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }


}

async fn process(socket : TcpStream) {
    // this connection let us read/write redis frames
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}