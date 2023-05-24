#[cfg(target_os = "vita")]
#[link(name = "pthread", kind = "static")]
extern "C" {}

fn main() {
    println!("Hello, world!");
}
