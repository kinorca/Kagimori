fn main() {
    tonic_build::configure()
        .compile_protos(&["proto/kagimori.proto", "proto/api.proto"], &["proto"])
        .unwrap();
}
