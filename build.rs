use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let protocol_wit_path = vtx_protocol::get_wit_path();
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=Using VTX Protocol definition from: {:?}", protocol_wit_path);

    let wit_content = fs::read_to_string(&protocol_wit_path)
        .expect("Failed to read WIT definition file");

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
        content = wit_content
    );

    let dest_path = Path::new(&out_dir).join("bindings.rs");

    // IO 操作：写入生成的 Rust 代码文件到 OUT_DIR
    fs::write(&dest_path, bindings_code).expect("Failed to write bindings.rs");
}
