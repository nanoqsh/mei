type Route = (&'static str, Page);

pub fn run(routes: &'static [Route]) {
    use {
        http_body_util::Full,
        hyper::{
            body::Incoming, header, http::HeaderValue, server::conn::http1, service, Method,
            Request, Response, StatusCode,
        },
        hyper_util::rt::TokioIo,
        std::{
            collections::HashMap,
            convert::Infallible,
            net::{Ipv4Addr, SocketAddr},
            sync::Arc,
        },
        tokio::{net::TcpListener, runtime, task},
    };

    let routes = {
        let routes: HashMap<_, _> = routes.iter().map(|(s, p)| (*s, p)).collect();
        Arc::new(routes)
    };

    let page = move |req: Request<Incoming>| {
        let mut not_found = Response::default();
        *not_found.status_mut() = StatusCode::NOT_FOUND;

        if req.method() != Method::GET {
            return not_found;
        }

        let Some(page) = routes.get(req.uri().path()) else {
            return not_found;
        };

        let mut res = Response::new(Full::new(page.body));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(page.content_type),
        );

        res
    };

    let run = || async {
        let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
        let listener = match TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(err) => return err,
        };

        println!("server listens on http://{addr}");

        loop {
            let stream = match listener.accept().await {
                Ok((stream, _)) => stream,
                Err(err) => return err,
            };

            let io = TokioIo::new(stream);
            let serve = service::service_fn({
                let page = page.clone();
                move |req| {
                    let res = page(req);
                    async { Ok::<_, Infallible>(res) }
                }
            });

            task::spawn(async move {
                if let Err(err) = http1::Builder::new().serve_connection(io, serve).await {
                    eprintln!("connection error: {err}");
                }
            });
        }
    };

    match runtime::Builder::new_current_thread().enable_io().build() {
        Ok(rt) => {
            let err = rt.block_on(run());
            eprintln!("io error: {err}");
        }
        Err(err) => eprintln!("failed to build tokio runtime: {err}"),
    }
}

pub struct Page {
    content_type: &'static str,
    body: &'static [u8],
}

impl Page {
    pub const fn html(body: &'static str) -> Self {
        Self {
            content_type: "text/html; charset=utf-8",
            body: body.as_bytes(),
        }
    }

    pub const fn wasm(body: &'static [u8]) -> Self {
        Self {
            content_type: "application/wasm",
            body,
        }
    }
}
