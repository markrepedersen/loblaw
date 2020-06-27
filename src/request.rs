use crate::algorithm::algorithm::Strategy;
use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Client, Response, Server,
    },
    std::net::SocketAddr,
};

pub struct RequestHandler {
    addr: SocketAddr,
    strategy: Strategy,
}

impl RequestHandler {
    pub fn new(addr: SocketAddr, strategy: Strategy) -> Self {
        Self { addr, strategy }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let forwarder_service = make_service_fn(move |_| async move {
            Ok::<_, hyper::Error>(service_fn(move |req| async move {
                match Client::new().request(req).await {
                    Ok(res) => Ok::<_, hyper::Error>(res),
                    Err(e) => Ok::<_, hyper::Error>(Response::new(Body::from(format!(
                        "Request failed due to '{}'",
                        e
                    )))),
                }
            }))
        });

        let server = Server::bind(&self.addr).serve(forwarder_service);

        println!("Listening on http://{}", self.addr);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }

        Ok(())
    }
}
