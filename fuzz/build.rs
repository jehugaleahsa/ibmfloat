fn main() {
    cc::Build::new()
        .file("ibm2ieee.c")
        .flag("-Wno-unused-function")
        .compile("ibm2ieee");
}
