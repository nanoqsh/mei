pub fn run(routes: &'static [Route]) {
    use {
        http_body_util::Full,
        hyper::{
            header, http::HeaderValue, server::conn::http1, service, Method, Request, Response,
            StatusCode,
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
        let routes: HashMap<_, _> = routes.iter().copied().collect();
        Arc::new(routes)
    };

    let page = move |req: Request<_>| match (req.method(), routes.get(req.uri().path())) {
        (&Method::GET, Some(page)) => {
            let mut res = Response::new(Full::new(page.body));
            res.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static(page.content_type),
            );

            Some(res)
        }
        _ => None,
    };

    let no_found = || {
        let mut res = Response::default();
        *res.status_mut() = StatusCode::NOT_FOUND;
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
                Ok((stream, _)) => TokioIo::new(stream),
                Err(err) => return err,
            };

            let serve = service::service_fn({
                let page = page.clone();
                move |req| {
                    let res = page(req).unwrap_or_else(no_found);
                    async { Ok::<_, Infallible>(res) }
                }
            });

            task::spawn(async {
                if let Err(err) = http1::Builder::new().serve_connection(stream, serve).await {
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

type Route = (&'static str, Page);

#[derive(Clone, Copy)]
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
