use anyhow::Result;
use evdev::{Device, EventSummary};

use crate::g13::button_from_code;

pub fn monitor(path: &std::path::Path) -> Result<()> {
    let mut device = Device::open(path)?;

    println!("Listening on {}", device.name().unwrap());

    loop {
        for event in device.fetch_events()? {
            if let EventSummary::Key(_, key, value) = event.destructure() {
                if let Some(button) = button_from_code(key.code()) {
                    match value {
                        1 => println!("{:?} Pressed", button),
                        0 => println!("{:?} Released", button),
                        _ => {}
                    }
                }
            }
        }
    }
}
