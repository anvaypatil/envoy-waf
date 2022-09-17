fn main() {
    let proto_file = [
        "./proto/logger.proto",
        "./proto/filter-constraints.proto"
    ];

    for file in proto_file {
        compile_proto_files(&file);
    }
}

fn compile_proto_files(proto_file: &str) {
    tonic_build::configure()
        .build_server(true)
        .out_dir("./src/generated")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
}