use crate::{algorithm::algorithm::ServerSelectionError, with_lock};
use {
    crate::{
        algorithm::algorithm::Algorithm,
        config::PersistenceType,
        Threadable,
        {algorithm::algorithm::Strategy, config::BackendConfig},
    },
    hyper::{body::HttpBody, service::Service, Body, Client, Request, Response, Server, Uri},
    std::{
        collections::HashMap,
        fmt,
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
        strategy: &Threadable<Strategy>,
        persistence_type: PersistenceType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let strategy = strategy.clone();
        let server = Server::bind(&self.addr).serve(MakeSvc::new(strategy, persistence_type));
        println!("Listening on http://{}", self.addr);
        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CookieError;

impl fmt::Display for CookieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request didn't contain any cookies.")
    }
}

#[derive(Debug, Clone)]
pub enum ServerMappingError {
    Cookie(CookieError),
    ServerSelection(ServerSelectionError),
}

impl fmt::Display for ServerMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerMappingError::Cookie(ref e) => e.fmt(f),
            ServerMappingError::ServerSelection(ref e) => e.fmt(f),
        }
    }
}

impl From<CookieError> for ServerMappingError {
    fn from(e: CookieError) -> Self {
        ServerMappingError::Cookie(e)
    }
}

impl From<ServerSelectionError> for ServerMappingError {
    fn from(e: ServerSelectionError) -> Self {
        ServerMappingError::ServerSelection(e)
    }
}

pub struct Svc {
    strategy: Threadable<Strategy>,
    persistence_type: PersistenceType,
    persistence_mappings: HashMap<String, BackendConfig>,
}

impl Svc {
    fn new(strategy: Threadable<Strategy>, persistence_type: PersistenceType) -> Self {
        Self {
            strategy,
            persistence_type,
            persistence_mappings: HashMap::new(),
        }
    }

    fn parse_cookies(&mut self, req: &Request<Body>, strategy: &mut Strategy) -> BackendConfig {
        match req.headers().get("cookie") {
            Some(cookies) => {
                let cookies = cookies.to_str().unwrap();
                if let Some(server) = self.persistence_mappings.get(cookies) {
                    server.clone()
                } else {
                    let server = strategy.server(req);
                    let cookie = cookies.to_string();

                    self.persistence_mappings.insert(cookie, server.clone());
                    server.clone()
                }
            }
            None => strategy.server(req).clone(),
        }
    }

    fn get_server(&mut self, req: &Request<Body>, strategy: &mut Strategy) -> BackendConfig {
        match self.persistence_type {
            PersistenceType::Cookie => self.parse_cookies(req, strategy),
            _ => unimplemented!("TODO: other persistence types."),
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
        let strategy = self.strategy.clone();
        let server = with_lock(strategy, |mut strategy| {
            self.get_server(&req, &mut strategy)
        });

        Box::pin(async move {
            let client = Client::new();
            let server_uri = {
                let uri = Uri::builder()
                    .scheme(server.scheme().as_str())
                    .authority(format!("{}:{}", server.ip(), server.port()).as_str())
                    .path_and_query(server.path().as_str())
                    .build();
                match uri {
                    Ok(uri) => uri,
                    Err(e) => panic!("Invalid URI: {}", e),
                }
            };

            println!("Request headers: {:#?}\n", req.headers());
            println!(
                "Forwarding request from '{}' to '{}'.",
                req.uri(),
                server_uri
            );

            *(req.uri_mut()) = server_uri;

            match client.request(req).await {
                Ok(mut res) => {
                    println!("Response: {}", res.status());
                    println!("Response headers: {:#?}\n", res.headers());
                    while let Some(next) = res.data().await {
                        let chunk = next.unwrap();
                        io::stdout().write_all(&chunk).await.unwrap();
                    }
                    Ok(res)
                }
                Err(e) => {
                    println!("{}", e);
                    Err(e)
                }
            }
        })
    }
}

pub struct MakeSvc {
    strategy: Threadable<Strategy>,
    persistence_type: PersistenceType,
}

impl MakeSvc {
    pub fn new(strategy: Threadable<Strategy>, persistence_type: PersistenceType) -> Self {
        Self {
            strategy,
            persistence_type,
        }
    }
}

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let strategy = self.strategy.clone();
        let persistence_type = self.persistence_type;
        let fut = async move { Ok(Svc::new(strategy, persistence_type)) };
        Box::pin(fut)
    }
}
