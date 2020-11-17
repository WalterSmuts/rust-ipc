extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["../protobuf/arithmetic.proto"])
        .include("../protobuf/")
        .run()
        .expect("protoc");
}
