use gui::Instance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gui: Instance = serde_json::from_str(include_str!("deserialization.json"))?;
    println!("{gui}");
    Ok(())
}
