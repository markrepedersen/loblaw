use {
    crate::{algorithm::algorithm::Algorithm, config, CONFIG, STRATEGY},
    hyper::{service::Service, Body, Client, Request, Response, Server},
    std::{
        future::Future,
        net::SocketAddr,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
};

pub struct RequestHandler {
    addr: SocketAddr,
}

impl RequestHandler {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let server = Server::bind(&self.addr).serve(MakeSvc {});
        println!("Listening on http://{}", self.addr);
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }

        Ok(())
    }
}

struct Svc;

impl Svc {
    fn get_server(&self) -> config::Server {
        let config = CONFIG.clone();
        let mut clone = STRATEGY.clone();

        if let Some(lock) = Arc::get_mut(&mut clone) {
            if let Ok(strategy) = lock.get_mut() {
                let servers = &config.servers;

                if let Some(server) = servers.get(strategy.server()) {
                    server.clone()
                } else {
                    panic!("Invalid server index.");
                }
            } else {
                panic!("Unable to get mutable reference to strategy.");
            }
        } else {
            panic!("Unable to acquire lock.");
        }
    }
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let server = self.get_server();
        Box::pin(async move {
            let client = Client::new();
            client.request(req).await
        })
    }
}

struct MakeSvc;

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let fut = async move { Ok(Svc {}) };
        Box::pin(fut)
    }
}
