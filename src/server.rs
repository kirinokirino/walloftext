
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use futures::TryStreamExt as _;

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
	let mut response = Response::new(Body::empty());

	match (req.method(), req.uri().path()) {
		(&Method::GET, "/") => {
			*response.body_mut() = Body::from(
			"Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`"
			);
		},
		(&Method::POST, "/echo") => {
			*response.body_mut() = req.into_body();
		},
		(&Method::POST, "/echo/uppercase") => {
    		// This is actually a new `futures::Stream`...
    		let mapping = req
    			.into_body().map_ok(|chunk| {
					chunk.iter()
						.map(|byte| byte.to_ascii_uppercase())
						.collect::<Vec<u8>>()
						});
    		*response.body_mut() = Body::wrap_stream(mapping);
		},
		(&Method::POST, "/echo/reverse") => {
	   		// Await the full body to be concatenated into a single `Bytes`...
			let full_body = hyper::body::to_bytes(req.into_body()).await?;

			let reversed = full_body.iter().rev().cloned().collect::<Vec<u8>>();
			*response.body_mut() = reversed.into();
		},
		_ => {
			*response.status_mut() = StatusCode::NOT_FOUND;	
		},
	}
	Ok(response)
}

pub fn spawn_server_thread() {
    // Configure a runtime that runs everything on the current thread
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    // Combine it with a `LocalSet,  which means it can spawn !Send futures...
    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, run_server());
}

async fn run_server() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
