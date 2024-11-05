extern crate dotenv;

fn main() {
    let dotenv_path = dotenv::dotenv().expect("failed to find .env file");
    println!("cargo:rerun-if-changed={}", dotenv_path.display());
    for (key, value) in dotenv::vars() {
        println!("cargo:rustc-env={key}={value}");
    }
}
