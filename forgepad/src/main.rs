mod events;
mod g13;

use anyhow::Result;

fn main() -> Result<()> {
    let devices = g13::discover()?;

    events::monitor(&devices.keypad)?;

    Ok(())
}
