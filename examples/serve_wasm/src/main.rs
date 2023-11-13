fn main() {
    use {
        hyper::{
            header, http::Error, service, Body, Method, Request, Response, Server, StatusCode,
        },
        std::{
            convert::Infallible,
            net::{Ipv4Addr, SocketAddr},
        },
        tokio::runtime::Builder,
    };

    fn page(req: Request<Body>) -> Result<Response<Body>, Error> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => Response::builder()
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(Body::from(include_str!("../static/index.html"))),
            (&Method::GET, "/greet_bg.wasm") => Response::builder()
                .header(header::CONTENT_TYPE, "application/wasm")
                .body(Body::from(&include_bytes!("../static/greet_bg.wasm")[..])),
            _ => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty()),
        }
    }

    let make = service::make_service_fn(|_| async {
        Ok::<_, Infallible>(service::service_fn(|req| async { page(req) }))
    });

    let fut = async {
        let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
        println!("start server at http://{addr}");
        _ = Server::bind(&addr).serve(make).await;
    };

    match Builder::new_current_thread().enable_io().build() {
        Ok(rt) => rt.block_on(fut),
        Err(err) => eprintln!("failed to build tokio runtime: {err}"),
    }
}
