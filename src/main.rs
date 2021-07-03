use std::process::Command;

use include_dir::{Dir, include_dir};
use rocket::fs::{FileServer, relative};

#[rocket::main]
async fn main() {

    #[cfg(debug_assertions)]
    Command::new("npm")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("run")
        .arg("dev")
        .spawn()
        .expect("failed to spawn npm");

    rocket::build()
        .mount("/", FileServer::from("public/"))
        .launch()
        .await
        .unwrap();
}
