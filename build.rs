use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    println!("cargo:rerun-if-changed=build.rs");

    let bindings_code = format!(
        r#####"
        wit_bindgen::generate!({{
            world: "plugin",
            inline: r####"
            {content}
            "####,
            pub_export_macro: true,
            default_bindings_module: "vtx_sdk::bindings",
        }});
        "#####,
        content = vtx_protocol::WIT_CONTENT
    );

    let dest_path = Path::new(&out_dir).join("bindings.rs");

    // IO 操作：写入生成的 Rust 代码文件到 OUT_DIR
    std::fs::write(&dest_path, bindings_code).expect("Failed to write bindings.rs");
}
