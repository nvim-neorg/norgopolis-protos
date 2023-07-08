fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("communication.proto")?;
    tonic_build::compile_protos("module.proto")?;
    Ok(())
}
