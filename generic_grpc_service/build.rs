
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/adac_remote.proto")?;
    Ok(())
}
