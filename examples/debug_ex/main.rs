#![crate_name = "version"]

fn main() {
    let version = liquidizers::version();
    println!("liquid-dsp version: {}", version);
}
