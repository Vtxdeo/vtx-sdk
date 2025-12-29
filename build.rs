use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let protocol_wit_path = vtx_protocol::get_wit_path();
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("vtx.wit");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=Using VTX Protocol definition from: {:?}", protocol_wit_path);

    fs::copy(&protocol_wit_path, &dest_path).unwrap_or_else(|err| {
        panic!(
            "Failed to copy WIT definition from {:?} to {:?}: {}",
            protocol_wit_path, dest_path, err
        )
    });
}
