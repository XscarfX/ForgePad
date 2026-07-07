use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct G13Devices {
    pub keypad: PathBuf,
    pub thumbstick: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G13Button {
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    G9,
    G10,
    G11,
    G12,
    G13,
    G14,
    G15,
    G16,
    G17,
    G18,
    G19,
    G20,
    G21,
    G22,
    M1,
    M2,
    M3,
    MR,
    Light,
    Thumb1,
    Thumb2,
}

pub fn button_from_code(code: u16) -> Option<G13Button> {
    match code {
        656 => Some(G13Button::G1),
        657 => Some(G13Button::G2),
        658 => Some(G13Button::G3),
        659 => Some(G13Button::G4),
        660 => Some(G13Button::G5),
        661 => Some(G13Button::G6),
        662 => Some(G13Button::G7),
        663 => Some(G13Button::G8),
        664 => Some(G13Button::G9),
        665 => Some(G13Button::G10),
        666 => Some(G13Button::G11),
        667 => Some(G13Button::G12),
        668 => Some(G13Button::G13),
        669 => Some(G13Button::G14),
        670 => Some(G13Button::G15),
        671 => Some(G13Button::G16),
        672 => Some(G13Button::G17),
        673 => Some(G13Button::G18),
        674 => Some(G13Button::G19),
        675 => Some(G13Button::G20),
        676 => Some(G13Button::G21),
        677 => Some(G13Button::G22),

        696 => Some(G13Button::M1),
        697 => Some(G13Button::M2),
        698 => Some(G13Button::M3),
        699 => Some(G13Button::MR),

        542 => Some(G13Button::Light),

        _ => None,
    }
}

pub fn discover() -> Result<G13Devices> {
    let mut keypad = None;
    let mut thumbstick = None;

    for entry in fs::read_dir("/dev/input/by-id")? {
        let path = fs::canonicalize(entry?.path())?;

        let Ok(device) = evdev::Device::open(&path) else {
            continue;
        };

        match device.name() {
            Some("Logitech G13 Gaming Keypad") => keypad = Some(path),
            Some("Logitech G13 Thumbstick") => thumbstick = Some(path),
            _ => {}
        }
    }

    Ok(G13Devices {
        keypad: keypad.ok_or(anyhow!("Keypad not found"))?,
        thumbstick: thumbstick.ok_or(anyhow!("Thumbstick not found"))?,
    })
}
