
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    tonic_build::configure()
        .out_dir("")
        .build_server(true)
        .build_client(true)
        .compile(&["./credits-manager-svc.proto"], &["."])?;
    Ok(())
}