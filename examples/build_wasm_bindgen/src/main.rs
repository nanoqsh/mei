fn main() {
    use file_server::Page;

    const ROUTES: [(&str, Page); 2] = [
        ("/", Page::html(include_str!("../static/index.html"))),
        (
            "/greet_bg.wasm",
            Page::wasm(include_bytes!("../static/greet_bg.wasm")),
        ),
    ];

    file_server::run(&ROUTES);
}
