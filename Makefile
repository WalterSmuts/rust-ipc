PB_GEN_CPP = gen/cpp/
RUST_DIR = rust-protobuf
RUST_OUT = $(RUST_DIR)/target/debug/

$(RUST_OUT)rust-protobuf: $(RUST_DIR)/src/main.rs
	cd rust-protobuf; cargo +nightly build

$(PB_GEN_CPP)/arithmetic.pb.cc $(PB_GEN_CPP)/arithmetic.pb.h: protobuf/arithmetic.proto
	mkdir -p gen
	protoc --cpp_out=gen protobuf/arithmetic.proto
