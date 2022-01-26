use gui::container::Container;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gui: Container = serde_json::from_str(include_str!("deserialization.json"))?;
    println!("{gui}");
    Ok(())
}
