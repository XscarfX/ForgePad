use anyhow::Result;
use evdev::Device;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct G13Devices {
    pub keypad: PathBuf,
    pub thumbstick: PathBuf,
}

pub fn discover() -> Result<G13Devices> {
    let mut keypad = None;
    let mut thumbstick = None;

    for entry in fs::read_dir("/dev/input/by-id")? {
        let entry = entry?;
        let path = fs::canonicalize(entry.path())?;

        let Ok(device) = Device::open(&path) else {
            continue;
        };

        let Some(name) = device.name() else {
            continue;
        };

        match name {
            "Logitech G13 Gaming Keypad" => keypad = Some(path),
            "Logitech G13 Thumbstick" => thumbstick = Some(path),
            _ => {}
        }
    }

    Ok(G13Devices {
        keypad: keypad.ok_or(anyhow::anyhow!("G13 keypad not found"))?,
        thumbstick: thumbstick.ok_or(anyhow::anyhow!("G13 thumbstick not found"))?,
    })
}
