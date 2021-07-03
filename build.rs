use std::process::Command;

fn main() {
    assert!(Command::new("npm")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("install")
        .output()
        .expect("Failed to run npm")
        .status
        .success());

    println!("cargo:rerun-if-changed=package.json");

    if !cfg!(debug_assertions) {
        let output = Command::new("npm")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .arg("run")
            .arg("build")
            .output()
            .expect("Failed to run npm");

        if !output.status.success() {
            panic!(
                "NPM exited with status {}\n{}",
                output.status.to_string(),
                String::from_utf8(output.stdout).unwrap()
            );
        }
    }

    println!("cargo:rerun-if-changed=web/");
}
