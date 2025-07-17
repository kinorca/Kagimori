fn main() {
    prost_build::compile_protos(&["proto/encryption-key.proto"], &["proto"]).unwrap();
}
