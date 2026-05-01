extern crate cc;
extern crate pkg_config;

use cmake::Config;

fn main() {
    let mut cc_builder = cc::Build::new();
    cc_builder
        .file("src/wrapper.cpp")
        .cpp(true)
        .flag_if_supported("-std=c++17");

    if pkg_config::Config::new().probe("opendht").is_err() {
        // try cmake
        let dst = Config::new("../opendht")
            .generator("Ninja")
            .define("CMAKE_INSTALL_LIBDIR", "lib")
            .define("OPENDHT_C", "ON")
            .define("OPENDHT_PYTHON", "OFF")
            .define("OPENDHT_PROXY_OPENSSL", "OFF")
            .define("OPENDHT_HTTP", "OFF")
            .define("OPENDHT_TOOLS", "OFF")
            .define("BUILD_TESTING", "OFF")
            .define("CMAKE_BUILD_TYPE", "MinSizeRel")
            .build();

        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        eprintln!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:include={}/include", dst.display());

        println!("cargo:rustc-link-lib=opendht");

        cc_builder.includes(Some(dst.join("include")));
    };

    println!(
        "cargo:rustc-flags=-lopendht -lgnutls -lssl -lcrypto -lnettle -lpthread -ljsoncpp -largon2 -lhttp_parser"
    );

    cc_builder.compile("dht-wrapper");

    println!("cargo:rustc-link-lib=static=dht-wrapper");
    println!("cargo:rustc-link-lib=stdc++");
}
