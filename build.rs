fn main() {
    println!("cargo:rustc-link-search=native=voicevox_core");
    println!("cargo:rustc-link-lib=static:+whole-archive=voicevox_core");
}
