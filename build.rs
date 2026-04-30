fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/services.proto"], // Path ke file proto
            &["proto"],                // Direktori file proto
        )?;
    Ok(())
}
