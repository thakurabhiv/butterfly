fn main() {
    println!("cargo:rerun-if-changed=./migrations/mysql");
    tauri_build::build();
}
