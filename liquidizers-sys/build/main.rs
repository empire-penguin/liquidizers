pub fn configure() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");

    liquidizers_src::Build::new().build();
}

fn main() {
    configure()
}
