use {
    crate::{
        algorithm::algorithm::RequestInfo,
        config::PersistenceType,
        with_read_lock, with_write_lock, Threadable,
        {
            algorithm::algorithm::{Algorithm, Strategy},
            config::BackendConfig,
        },
    },
    actix_web::{
        client::{Client, ClientResponse},
        http::{header, Cookie},
        middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    },
    std::{
        collections::hash_map::{DefaultHasher, HashMap},
        hash::{Hash, Hasher},
        net::SocketAddr,
        sync::{Arc, RwLock},
    },
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

    async fn get_server(
        strategy: Threadable<Strategy>,
        mappings: Threadable<HashMap<String, BackendConfig>>,
        req_info: &RequestInfo,
        session_id: &String,
    ) -> BackendConfig {
        let server = with_read_lock(mappings.clone(), |mappings| {
            mappings.get(session_id).cloned()
        });
        match server {
            Some(server) => {
                println!("[Cached] Found server: {}.", server.ip());
                server
            },
            None => {
                let mut strategy = strategy.write().expect("Couldn't acquire the lock.");
                let server = strategy
                    .server(req_info)
                    .await
                    .expect("Unable to retrieve server.");
                with_write_lock(mappings, |mappings| {
                    mappings.insert(session_id.clone(), server.clone())
                });
                println!("[No cache] Found server: {}.", server.ip());
                server
            }
        }
    }

    /// Forward `req` to a given server based on a previously chosen strategy.
    async fn forward(
        req: HttpRequest,
        body: web::Bytes,
        client: web::Data<Client>,
        strategy: web::Data<Threadable<Strategy>>,
        mappings: web::Data<Threadable<HashMap<String, BackendConfig>>>,
    ) -> Result<HttpResponse, Error> {
        let strategy = strategy.get_ref().clone();
        let mappings = mappings.get_ref().clone();
        let client_uri = if let Some(addr) = req.peer_addr() {
            addr.to_string()
        } else {
            String::from("")
        };
        let session_id = req.get_session_id(&client_uri, &req.get_server_host());
        let req_info = RequestInfo::new(req.uri().clone(), req.connection_info().clone());
        let server = Self::get_server(strategy, mappings, &req_info, &session_id).await;
        let uri = server.uri()?;
        let mut forwarded_response = client
            .request_from(uri, req.head())
            .no_decompress()
            .header(header::FORWARDED, client_uri)
            .send_body(body)
            .await
            .map_err(Error::from)?;
        let mut res = HttpResponse::build(forwarded_response.status());

        if !forwarded_response.has_cookie(COOKIE_SESSION_KEY) {
            let cookie = Cookie::build(COOKIE_SESSION_KEY, session_id)
                .max_age(30)
                .http_only(true)
                .finish();
            res.cookie(cookie);
        }

        Ok(res.body(forwarded_response.body().await?))
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let strat = self.strategy.clone();
        let mappings = self.persistence_mappings.clone();
        println!("Waiting for packets on '{}'.", &self.addr);
        HttpServer::new(move || {
            App::new()
                .data(Client::new())
                .data(strat.clone())
                .data(mappings.clone())
                .wrap(middleware::Logger::default())
                .default_service(web::route().to(Self::forward))
        })
        .bind(self.addr)?
        .run()
        .await?;

        Ok(())
    }
}

trait HasCookie {
    fn has_cookie(&mut self, cookie_key: &str) -> bool;
}

impl<T> HasCookie for ClientResponse<T> {
    fn has_cookie(&mut self, cookie_key: &str) -> bool {
        self.headers()
            .get_all(header::COOKIE)
            .into_iter()
            .map(|value| Cookie::parse(value.to_str().unwrap()))
            .any(|cookie| {
                if let Ok(cookie) = cookie {
                    cookie.name() == cookie_key
                } else {
                    false
                }
            })
    }
}

trait RequestProxy {
    fn get_session_id(&self, client_host: &String, server_host: &String) -> String;
    fn get_server_host(&self) -> String;
}

impl RequestProxy for HttpRequest {
    fn get_session_id(&self, client_host: &String, server_host: &String) -> String {
        self.headers()
            .get_all(header::COOKIE)
            .into_iter()
            .map(|value| Cookie::parse(value.to_str().unwrap()))
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

    fn get_server_host(&self) -> String {
        self.headers()
            .get("host")
            .expect("Request should have HOST header.")
            .to_str()
            .unwrap()
            .to_string()
    }
}
