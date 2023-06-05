fn main() {
    tonic_build::compile_protos("rpc/helloworld.proto").unwrap();
}
