use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let work_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let work_path = work_dir.to_str().unwrap();
    println!("cargo:rustc-link-search={}/lib", work_path);
    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());

    let mut cc_builder = cc::Build::new();
    cc_builder.cpp(true)
        .flag("-std=c++11")
        .flag("-march=armv7")
        .flag(&format!("-I{}", work_path))
        .flag(&format!("-I{}/include", work_path));


    let cc_compiler = cc_builder.get_compiler();
    let cc_sysroot = PathBuf::from(
        String::from_utf8(
            cc_compiler
                .to_command()
                .arg("-print-sysroot")
                .output()
                .unwrap()
                .stdout,
        )
            .unwrap()
            .trim(),
    )
        .canonicalize()
        .unwrap();

    let toolchain_dir = cc_sysroot.parent().unwrap().parent().unwrap();

    let _toolchain_gcc_include_dir = toolchain_dir
        .join("lib/gcc/arm-rockchip830-linux-uclibcgnueabihf/8.3.0/include")
        .canonicalize()
        .unwrap();

    let _toolchain_cpp_include_dir = toolchain_dir
        .join("arm-rockchip830-linux-uclibcgnueabihf/include/c++/8.3.0")
        .canonicalize()
        .unwrap();

    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=atomic");

}