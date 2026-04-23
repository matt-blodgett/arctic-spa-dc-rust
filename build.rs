use std::fs;
use std::path::PathBuf;


const PROTO_DIR_IN: &str = "src/proto_schemas";
const PROTO_DIR_OUT: &str = "src/proto";


fn main() {
    // Create output directory if it doesn't exist
    fs::create_dir_all(PROTO_DIR_OUT)
        .expect("Failed to create output directory");

    // Enumerate all .proto files in the directory
    let proto_files: Vec<PathBuf> = fs::read_dir(PROTO_DIR_IN)
        .expect("Failed to read proto directory")
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension().map_or(false, |ext| ext == "proto") {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    protobuf_codegen::Codegen::new()
        .pure()
        .includes(&[PROTO_DIR_IN])
        .inputs(&proto_files)
        .out_dir(PROTO_DIR_OUT)
        .run()
        .expect("Codegen failed");
}
