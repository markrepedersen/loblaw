use {
    crate::{
        algorithm::algorithm::{Algorithm, Strategy},
        BackendConfig,
    },
    cookie,
    hyper::{header, Body, Request, Response},
    std::{
        collections::hash_map::{DefaultHasher, HashMap},
        hash::{Hash, Hasher},
    },
    time::Duration,
};

#[derive(Hash)]
struct User {
    client_host: String,
    server_host: String,
}

impl User {
    fn generate_unique_id(user: User) -> String {
        let mut s = DefaultHasher::new();
        user.hash(&mut s);
        s.finish().to_string()
    }
}

pub struct Cookie;

impl Cookie {
    pub fn has_cookie(response: &Response<Body>, cookie_key: &str) -> bool {
        response
            .headers()
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

    pub fn set_cookie(
        response: &mut Response<Body>,
        client_host: String,
        server_host: String,
        cookie_key: &str,
    ) {
        println!("Response: {:?}", response);
        let session_id = User::generate_unique_id(User {
            client_host,
            server_host,
        });
        let cookie = cookie::Cookie::build(cookie_key, session_id)
            .max_age(Duration::seconds(30))
            .http_only(true)
            .finish();
        let val = header::HeaderValue::from_str(&cookie.encoded().to_string()).unwrap();
        response.headers_mut().insert(header::SET_COOKIE, val);
    }

    pub fn parse_cookies(
        req: &Request<Body>,
        strategy: &mut Strategy,
        mappings: &mut HashMap<String, BackendConfig>,
    ) -> BackendConfig {
        println!("Request: {:?}", req);
        match req.headers().get(header::COOKIE) {
            Some(cookies) => {
                let cookies = cookies.to_str().unwrap();
                if let Some(server) = mappings.get(cookies) {
                    server.clone()
                } else {
                    let server = strategy.server(req);
                    let cookie = cookies.to_string();

                    mappings.insert(cookie, server.clone());
                    server.clone()
                }
            }
            None => strategy.server(req).clone(),
        }
    }
}
