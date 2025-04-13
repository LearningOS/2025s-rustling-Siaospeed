use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("env_config.rs");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let content = format!("pub const ENV_TIMESTAMP: u64 = {};", timestamp);

    let mut f = fs::File::create(&dest_path).unwrap();
    f.write_all(content.as_bytes()).unwrap();

    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // For tests8.rs
    println!("cargo:rustc-cfg=feature=\"pass\"");
}