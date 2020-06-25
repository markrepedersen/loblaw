use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Client, Request, Response, Server,
    },
    std::{error::Error, net::SocketAddr},
};

async fn forward_request(req: Request<Body>) -> hyper::Result<Response<Body>> {
    /// TODO: Pick the server that the request should be sent to.
    let res = Client::new().request(req).await?;
    /// TODO: Switch IPs so response goes to correct place.
    Ok(res)
}

pub async fn handle_requests(addr: &SocketAddr) -> Result<(), Box<dyn Error>> {
    let server = Server::bind(addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(forward_request))
    }));

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
