fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    let out_path = std::path::Path::new(&out_dir).join("patients_descriptor.bin");

    tonic_prost_build::configure()
        .file_descriptor_set_path(&out_path)
        .compile_protos(&["proto/patients.proto"], &["proto"])?;

    Ok(())
}
