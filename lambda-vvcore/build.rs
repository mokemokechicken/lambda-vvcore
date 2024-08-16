fn main() {
    println!("cargo:rustc-link-search=native=/opt/voicevox/voicevox_core");
    println!("cargo:rustc-link-lib=dylib=voicevox_core");
}