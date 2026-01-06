fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(true)
        .compile_protos(&["proto/store.proto"], &["proto"])?;

    Ok(())
}
