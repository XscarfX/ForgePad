mod g13;

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    println!("ForgePad v0.1.0-alpha");

    let devices = g13::discover()?;

    println!("Keypad    : {}", devices.keypad.display());
    println!("Thumbstick: {}", devices.thumbstick.display());

    Ok(())
}
