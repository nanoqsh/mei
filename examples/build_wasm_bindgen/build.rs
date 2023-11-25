fn main() {
    use {
        askama::Template,
        mei::{OptLevel, Target},
    };

    // TODO
    println!("cargo:rerun-if-changed=.");

    let mut make_greet = mei::cargo_build();
    make_greet
        .manifest("greet")
        .target(Target::WASM32_UNKNOWN_UNKNOWN);

    if OptLevel::is_optimized() {
        make_greet.profile("production");
    }

    make_greet.spawn();
    let greet = mei::artifact("greet.wasm");
    let js_out = mei::target_dir().join("js");

    mei::tool("wasm-bindgen")
        .args(["--target", "no-modules"])
        .arg("--out-dir")
        .arg(&js_out)
        .arg("--no-typescript")
        .arg(make_greet.path_of(&greet))
        .spawn();

    let index = {
        #[derive(Template)]
        #[template(path = "index.html")]
        struct Index {
            js: String,
        }

        let js_path = js_out.join("greet.js");
        let js = mei::read_to_string(js_path);
        Index { js }
    };

    mei::write(
        "static/index.html",
        index.render().expect("render index.html"),
    );

    let greet_bg_path = js_out.join("greet_bg.wasm");
    if cfg!(feature = "wasm-opt") && OptLevel::is_optimized() {
        mei::tool("wasm-opt")
            .arg("-Os")
            .arg(&greet_bg_path)
            .args(["-o", "static/greet_bg.wasm"])
            .spawn();
    } else {
        mei::copy(&greet_bg_path, "static/greet_bg.wasm");
    }
}
