fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("OUT_DIR", "src");
    tonic_build::compile_protos("proto/gateway.proto")?;
    Ok(())
}
