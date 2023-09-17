fn main() {
    println!("cargo:rustc-link-search=native=voicevox_core");
    println!("cargo:rustc-link-lib=static=voicevox_core");
}
