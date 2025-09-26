use std::{env, path::Path};

fn main() {
    let pq_lib = env::var("PQ_LIB_DIR").unwrap_or_else(|_| r"C:\Program Files\PostgreSQL\17\lib".into());
    let pq_inc = env::var("PQ_INCLUDE_DIR").unwrap_or_else(|_| r"C:\Program Files\PostgreSQL\17\include".into());

    println!("cargo:warning=PQ_LIB_DIR={pq_lib}");
    println!("cargo:warning=PQ_INCLUDE_DIR={pq_inc}");

    assert!(Path::new(&pq_lib).join("libpq.lib").exists(), "libpq.lib not found in PQ_LIB_DIR: {pq_lib}");
    assert!(Path::new(&pq_inc).join("libpq-fe.h").exists(), "libpq-fe.h not found in PQ_INCLUDE_DIR: {pq_inc}");

    println!("cargo:rustc-link-search=native={pq_lib}");
    println!("cargo:rustc-link-lib=pq");

    println!("cargo:rerun-if-env-changed=PQ_LIB_DIR");
    println!("cargo:rerun-if-env-changed=PQ_INCLUDE_DIR");
}
