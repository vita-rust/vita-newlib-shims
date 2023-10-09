use std::{collections::HashMap, path::Path, process::Command};

const FEATURES: &[&str] = &["socketpair", "pipe2"];

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-env-changed=VITASDK");

    let vitasdk = std::env::var("VITASDK").expect("VITASDK not set");
    let vitasdk = Path::new(&vitasdk);

    let libc_a = vitasdk.join("arm-vita-eabi").join("lib").join("libc.a");
    let nm = vitasdk.join("bin").join("arm-vita-eabi-nm");

    println!("cargo:rerun-if-changed={}", libc_a.display());
    println!("cargo:rerun-if-changed={}", nm.display());

    let nm_result = Command::new(nm)
        .arg("-n")
        .arg(libc_a)
        .output()
        .expect("nm failed")
        .stdout;
    let nm_result = String::from_utf8_lossy(&nm_result);
    let nm_result = nm_result.lines();

    let mut features = FEATURES
        .iter()
        .map(|f| (*f, false))
        .collect::<HashMap<_, _>>();

    for line in nm_result {
        for (feature, enabled) in &mut features {
            if line == format!("00000000 T {}", feature) {
                *enabled = true;
            }
        }
    }

    for (feature, _) in features.iter().filter(|(_, enabled)| !**enabled) {
        println!("cargo:rustc-cfg=feature=\"{}\"", feature);
    }
}
