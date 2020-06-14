// use tokio::io::AsyncWriteExt;
// use tokio::net::TcpStream;
//
use warp::Filter;

// > curl localhost:8080/hello/async
// Hello, async
#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    //let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    //let result = stream.write(b"hello async\n").await;
    //println!("wrote to stream; success={:?}", result.is_ok());

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
