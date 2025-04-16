fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Uncomment this to have the gRPC code generated at compile time
    // Otherwise use the xtask

    // tonic_build::configure()
    //     .build_server(true)
    // puts the code in the repo
    // Comment out to put the code in the OUT_DIR
    //     .out_dir("src/api/grpc/generated")

    //     .compile_protos(&["proto/hello.proto"], &["proto"])?;
    // println!("cargo:rerun-if-changed=proto/hello.proto");
    Ok(())
}
