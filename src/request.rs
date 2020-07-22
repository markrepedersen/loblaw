use {
    crate::{
        config::PersistenceType,
        cookie::Cookie,
        with_write_lock, Threadable,
        {algorithm::algorithm::Strategy, config::BackendConfig},
    },
    hyper::{
        header,
        server::conn::AddrStream,
        service::{make_service_fn, service_fn},
        Body, Client, Request, Response, Server, Uri,
    },
    std::{
        collections::HashMap,
        convert::Infallible,
        net::SocketAddr,
        sync::{Arc, RwLock},
    },
};

static COOKIE_SESSION_KEY: &'static str = "session";

pub struct RequestHandler {
    addr: SocketAddr,
    strategy: Threadable<Strategy>,
    persistence_type: PersistenceType,
    persistence_mappings: Arc<RwLock<HashMap<String, BackendConfig>>>,
}

impl RequestHandler {
    pub fn new(
        addr: SocketAddr,
        persistence_type: PersistenceType,
        strategy: Threadable<Strategy>,
    ) -> Self {
        Self {
            addr,
            persistence_type,
            strategy,
            persistence_mappings: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn choose_server(
        req: &Request<Body>,
        strategy: &mut Strategy,
        typ: PersistenceType,
        mappings: Arc<RwLock<HashMap<String, BackendConfig>>>,
    ) -> BackendConfig {
        match typ {
            PersistenceType::Cookie => with_write_lock(mappings, |ref mut mappings| {
                Cookie::parse_cookies(req, strategy, mappings)
            }),
            _ => unimplemented!("TODO: other persistence types."),
        }
    }

    async fn forward_request(
        req: Request<Body>,
        client: SocketAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let client_host = format!("{}", client.ip());
        let server_host = req
            .headers()
            .get("host")
            .expect("Request should have HOST header.")
            .to_str()
            .unwrap()
            .to_string();

        let mut res = Client::new().request(req).await;

        if let Ok(ref mut response) = res {
            if !Cookie::has_cookie(response, COOKIE_SESSION_KEY) {
                Cookie::set_cookie(response, client_host, server_host, COOKIE_SESSION_KEY);
            }
        }

        res
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let server = Server::bind(&self.addr).serve(make_service_fn(move |conn: &AddrStream| {
            let addr = conn.remote_addr();
            let typ = self.persistence_type;
            let strategy = self.strategy.clone();
            let mappings = self.persistence_mappings.clone();
            async move {
                let client = addr.clone();
                Ok::<_, Infallible>(service_fn(move |mut req| {
                    let server = with_write_lock(strategy.clone(), |mut strategy| {
                        Self::choose_server(&req, &mut strategy, typ, mappings.clone())
                    });
                    req.set_uri(&server);
                    Self::forward_request(req, client)
                }))
            }
        }));

        println!("Listening on http://{}", self.addr);

        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }

        Ok(())
    }
}

trait Proxy {
    fn set_uri(&mut self, server: &BackendConfig);
    fn set_headers(&mut self, map: HashMap<String, String>);
}

impl Proxy for Request<Body> {
    fn set_uri(&mut self, server: &BackendConfig) {
        *(self.uri_mut()) = Uri::builder()
            .scheme(server.scheme().as_str())
            .authority(format!("{}:{}", server.ip(), server.port()).as_str())
            .path_and_query(server.path().as_str())
            .build()
            .unwrap();
    }

    fn set_headers(&mut self, map: HashMap<String, String>) {
        match self.headers_mut().get(header::FORWARDED) {
            Some(_) => {}
            None => {}
        };
    }
}
