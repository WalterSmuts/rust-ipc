PB_GEN_CPP = gen/cpp
RUST_DIR = rust-protobuf
RUST_OUT = $(RUST_DIR)/target/debug
CPP_DIR = cpp-protobuf
CPP_OUT = cpp-build-aritfacts

all: $(CPP_OUT)/client $(RUST_OUT)/rust-protobuf

$(CPP_OUT)/client: $(CPP_DIR)/main.cpp $(PB_GEN_CPP)/protobuf/arithmetic.pb.cc $(PB_GEN_CPP)/protobuf/arithmetic.pb.h
	mkdir -p $(CPP_OUT)
	g++ -I$(PB_GEN_CPP) -o $(CPP_OUT)/client $(CPP_DIR)/main.cpp $(PB_GEN_CPP)/protobuf/arithmetic.pb.cc -lprotobuf

$(RUST_OUT)/rust-protobuf: $(RUST_DIR)/src/main.rs
	cd rust-protobuf; cargo +nightly build

$(PB_GEN_CPP)/protobuf/arithmetic.pb.cc $(PB_GEN_CPP)/protobuf/arithmetic.pb.h: protobuf/arithmetic.proto
	mkdir -p $(PB_GEN_CPP)
	protoc --cpp_out=$(PB_GEN_CPP) protobuf/arithmetic.proto
