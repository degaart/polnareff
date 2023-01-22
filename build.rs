use std::{env, path::Path, process::Command};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=stub/stub.asm");
    let dest_path = Path::new(&out_dir).join("stub.o");
    Command::new("nasm")
        .arg("-o")
        .arg(&dest_path)
        .arg("-felf32")
        .arg("stub/stub.asm")
        .status()
        .unwrap();
    println!("cargo:rustc-link-arg={}", dest_path.display());
}

