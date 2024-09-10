use db::{db_task, Cmd};
use hyper::{body::Incoming, server::conn::http1, service::service_fn, Request};
use hyper_util::rt::TokioIo;
use todo_server::{db, router::router};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let addr = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let (tx, rx) = mpsc::channel::<Cmd>(128);
    
    tokio::task::spawn(db_task(rx));
    
    loop {
        let stream = match addr.accept().await {
            Ok((stream, _)) => stream,
            Err(e) => {
                println!("[error] {:?}", e);
                continue;
            }
        };
        let io = TokioIo::new(stream);

        let tx_clone = tx.clone();

        let service = service_fn(move |req: Request<Incoming>| {
            let tx_clone = tx_clone.clone();

            async move {
                let response = router(req, tx_clone).await;
                Ok::<_, hyper::Error>(response)
            }
        });

        tokio::task::spawn(async move {
            if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                println!("[error]: {:?}", e);
            }
        });
    }
}
