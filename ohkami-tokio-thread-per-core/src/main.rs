use ohkami::prelude::*;

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

fn ohkami() -> Ohkami {    
    Ohkami::new((
        "/".GET(async || { Response::OK() }),
        "/user".POST(async || { Response::OK() }),
        "/user/:id".GET(async |Path(id): Path<String>| { id }),
    ))
}

async fn serve(o: Ohkami) -> std::io::Result<()> {
    let socket = tokio::net::TcpSocket::new_v4()?;
    socket.set_reuseport(true)?;
    socket.set_reuseaddr(true)?;
    socket.set_nodelay(true)?;

    socket.bind("0.0.0.0:3000".parse().unwrap())?;
    o.howl(socket.listen(4096)?).await;

    Ok(())
}

fn main() {
    let ncpus = std::thread::available_parallelism().map_or(1, |x| x.get());
    
    for _ in 0..dbg!(ncpus - 1/* for main thread */) {
        std::thread::spawn(|| {
            runtime().block_on(async {
                serve(ohkami()).await.expect("serving error")
            });
        });
    }
    runtime().block_on(async {
        serve(ohkami()).await.expect("serving error")
    });
}