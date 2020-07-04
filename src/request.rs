use {
    crate::{algorithm::algorithm::Algorithm, status, Threadable},
    hyper::{body::HttpBody, service::Service, Body, Client, Request, Response, Server, Uri},
    std::{
        future::Future,
        net::SocketAddr,
        pin::Pin,
        task::{Context, Poll},
    },
    tokio::io::{self, AsyncWriteExt as _},
};

pub struct RequestHandler {
    addr: SocketAddr,
}

impl RequestHandler {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn run(
        &self,
        config: Threadable<status::Global>,
        servers: Threadable<Vec<status::Server>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config = config.clone();
        let servers = servers.clone();
        let server = Server::bind(&self.addr).serve(MakeSvc { config, servers });
        println!("Listening on http://{}", self.addr);
        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }

        Ok(())
    }
}

pub struct Svc {
    config: Threadable<status::Global>,
    servers: Threadable<Vec<status::Server>>,
}

impl Svc {
    fn get_server(&self) -> status::Server {
        let servers = {
            let s = self.servers.clone();
            let s = s.lock().unwrap();
            s.clone()
        };
        let c = self.config.clone();
        let mut c = c.lock().unwrap();
        match servers.get(c.strategy_mut().server()) {
            Some(server) => server.clone(),
            None => panic!("Invalid server index."),
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
            let server_uri = format!("{}:{}{}", server.ip(), server.port(), server.path())
                .as_str()
                .parse::<Uri>()
                .unwrap();

            *(req.uri_mut()) = server_uri;

            match client.request(req).await {
                Ok(mut res) => {
                    println!("Response: {}", res.status());
                    println!("Headers: {:#?}\n", res.headers());
                    while let Some(next) = res.data().await {
                        let chunk = next.unwrap();
                        io::stdout().write_all(&chunk).await.unwrap();
                    }
                    Ok(res)
                }
                Err(e) => Err(e),
            }
        })
    }
}

pub struct MakeSvc {
    config: Threadable<status::Global>,
    servers: Threadable<Vec<status::Server>>,
}

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let config = self.config.clone();
        let servers = self.servers.clone();
        let fut = async move { Ok(Svc { config, servers }) };
        Box::pin(fut)
    }
}
