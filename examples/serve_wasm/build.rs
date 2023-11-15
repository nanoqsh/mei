use {
    mei::{OptLevel, Spawn, Target},
    sailfish::TemplateOnce,
};

fn main() {
    // TODO
    println!("cargo:rerun-if-changed=.");

    let mut make_greet = mei::cargo()
        .manifest("greet")
        .target(Target::WASM32_UNKNOWN_UNKNOWN);

    make_greet.spawn();
    let greet = mei::artifact("greet.wasm");
    let js_dir = mei::subdir("js");

    // > wasm-bindgen --target no-modules --out-dir {js_dir} --no-typescript {make_greet.path_of(&greet)}
    mei::tool("wasm-bindgen")
        .args(["--target", "no-modules"])
        .arg("--out-dir")
        .arg(&js_dir)
        .arg("--no-typescript")
        .arg(make_greet.path_of(&greet))
        .spawn();

    let js_path = js_dir.join("greet.js");
    let js = mei::read_to_string(&js_path);

    mei::write("static/index.html", {
        let index = Index { js };
        index.render_once().expect("render index.html")
    });

    let greet_bg_path = js_dir.join("greet_bg.wasm");
    if cfg!(feature = "wasm-opt") && OptLevel::current() > OptLevel::N1 {
        // > wasm-opt -Os {greet_bg_path} -o static/greet_bg.wasm
        mei::tool("wasm-opt")
            .arg("-Os")
            .arg(&greet_bg_path)
            .args(["-o", "static/greet_bg.wasm"])
            .spawn();
    } else {
        mei::copy(&greet_bg_path, "static/greet_bg.wasm");
    }
}

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Index {
    js: String,
}
