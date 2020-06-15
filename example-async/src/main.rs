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

    // let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let hello = hello().and(name()).and_then(greet_handler);
    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}

fn hello() -> warp::filters::BoxedFilter<()> {
    warp::path("hello").boxed()
}

fn name() -> warp::filters::BoxedFilter<(Name,)> {
    warp::path::param().boxed()
}

async fn greet_handler(name: Name) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("hello {}", name);
    Ok(warp::reply::html(reply))
}

#[derive(Clone, Debug)]
struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, String> {
        let size = name.chars().count();
        if size < 1 || size > 10 {
            return Err("required: less than 10 strings".to_string());
        }
        if name.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err("enable: A-Z, a-z.".to_string());
        }
        Ok(Name(name.to_string()))
    }
}

impl std::str::FromStr for Name {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Name::new(s)
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_name() {
        let ok_value = "Async";
        assert!(Name::new(ok_value).is_ok());
    }
} /* test */
