use {
    crate::{
        config::PersistenceType,
        with_write_lock, Threadable,
        {
            algorithm::algorithm::{Algorithm, Strategy},
            config::BackendConfig,
        },
    },
    hyper::{
        header,
        server::conn::AddrStream,
        service::{make_service_fn, service_fn},
        Body, Client, Request, Response, Server, Uri,
    },
    std::{
        collections::hash_map::{DefaultHasher, HashMap},
        hash::{Hash, Hasher},
        net::SocketAddr,
        sync::{Arc, RwLock},
    },
    time::Duration,
};

static COOKIE_SESSION_KEY: &'static str = "session";

#[derive(Hash)]
pub struct User {
    client_host: String,
    server_host: String,
}

impl User {
    fn generate_unique_id(client_host: String, server_host: String) -> String {
        let user = User {
            client_host,
            server_host,
        };
        let mut s = DefaultHasher::new();
        user.hash(&mut s);
        s.finish().to_string()
    }
}

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

    /// Forward `req` to a given server based on a previously chosen strategy.
    ///
    /// # Note:
    /// This should be inside the Request trait, however, async trait functions are unstable as of writing this.
    /// Using the `async-trait` crate would mean heap allocation, which is not desired due to the frequency of calling this method.
    async fn forward_request(
        req: Request<Body>,
        session_id: String,
    ) -> Result<Response<Body>, hyper::Error> {
        let mut res = Client::new().request(req).await;

        if let Ok(ref mut response) = res {
            if !response.has_cookie(COOKIE_SESSION_KEY) {
                response.set_cookie(session_id, COOKIE_SESSION_KEY);
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
                Ok::<_, hyper::Error>(service_fn(move |mut req| {
                    let session_id =
                        req.get_session_id(client.ip().to_string(), req.get_server_host());
                    dbg!(&session_id);
                    println!();
                    let server = with_write_lock(mappings.clone(), |mappings| {
                        mappings
                            .get(&session_id)
                            .cloned()
                            .or_else(|| {
                                let server = with_write_lock(strategy.clone(), |strategy| {
                                    strategy.server(&req).clone()
                                });
                                mappings.insert(session_id.clone(), server.clone());
                                Some(server)
                            })
                            .expect("A server should have been chosen.")
                    });

                    req.set_uri(&server);
                    req.set_headers(&client);

                    Self::forward_request(req, session_id)
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

trait ResponseProxy {
    fn set_cookie(&mut self, session_id: String, cookie_key: &str);
    fn has_cookie(&mut self, cookie_key: &str) -> bool;
}

impl ResponseProxy for Response<Body> {
    fn has_cookie(&mut self, cookie_key: &str) -> bool {
        self.headers()
            .get_all(header::COOKIE)
            .iter()
            .map(|value| cookie::Cookie::parse(value.to_str().unwrap()))
            .any(|cookie| {
                if let Ok(cookie) = cookie {
                    cookie.name() == cookie_key
                } else {
                    false
                }
            })
    }

    fn set_cookie(&mut self, session_id: String, cookie_key: &str) {
        let cookie = cookie::Cookie::build(cookie_key, session_id)
            .max_age(Duration::seconds(30))
            .http_only(true)
            .finish();
        let val = header::HeaderValue::from_str(&cookie.encoded().to_string()).unwrap();
        self.headers_mut().insert(header::SET_COOKIE, val);
    }
}

trait RequestProxy {
    fn get_session_id(&self, client_host: String, server_host: String) -> String;
    fn set_uri(&mut self, server: &BackendConfig);
    fn set_headers(&mut self, addr: &SocketAddr);
    fn get_server_host(&self) -> String;
}

impl RequestProxy for Request<Body> {
    fn get_session_id(&self, client_host: String, server_host: String) -> String {
        self.headers()
            .get_all(header::COOKIE)
            .iter()
            .map(|value| cookie::Cookie::parse(value.to_str().unwrap()))
            .find(|val| {
                if let Ok(cookie) = val {
                    cookie.name() == COOKIE_SESSION_KEY
                } else {
                    false
                }
            })
            .and_then(|cookie| {
                if let Ok(cookie) = cookie {
                    Some(cookie.value().to_string())
                } else {
                    Some(User::generate_unique_id(
                        client_host.clone(),
                        server_host.clone(),
                    ))
                }
            })
            .or_else(|| {
                Some(User::generate_unique_id(
                    client_host.clone(),
                    server_host.clone(),
                ))
            })
            .expect("At least one session ID should have been chosen.")
    }

    fn set_uri(&mut self, server: &BackendConfig) {
        *(self.uri_mut()) = Uri::builder()
            .scheme(server.scheme().as_str())
            .authority(format!("{}:{}", server.ip(), server.port()).as_str())
            .path_and_query(server.path().as_str())
            .build()
            .unwrap();
    }

    fn set_headers(&mut self, addr: &SocketAddr) {
        let forwarded_val = header::HeaderValue::from_str(&addr.ip().to_string()).unwrap();
        self.headers_mut().insert(header::FORWARDED, forwarded_val);
    }

    fn get_server_host(&self) -> String {
        self.headers()
            .get("host")
            .expect("Request should have HOST header.")
            .to_str()
            .unwrap()
            .to_string()
    }
}
