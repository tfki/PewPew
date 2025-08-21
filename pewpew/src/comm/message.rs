use hecs::Entity;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum SerialToGuiKind {
    Reload,
    Shot,
}

#[derive(Debug, Clone)]
pub struct SerialToGui {
    pub sensortag_id: u16,
    pub timestamp: u32,
    pub ammo: u8,
    pub ammo_max: u8,
    pub kind: SerialToGuiKind,
}

#[derive(Debug, Clone)]
pub struct SerialToHitReg {
    pub sensortag_id: u16,
    pub timestamp: u32,
    pub value_raw: u16,
}

#[derive(Debug, Clone)]
pub enum FromSerial {
    ToGui(SerialToGui),
    ToHitReg(SerialToHitReg),
}

#[derive(Debug, Clone)]
pub enum GuiToHitreg {
    FlashingSequenceStart{num_frames: u32, sequences: Vec<(Entity, Vec<bool>)>},
    Frame(SystemTime),
}

#[derive(Debug, Clone)]
pub enum HitregToGui {
    Result(Option<Entity>),
}

#[derive(Debug, Clone)]
pub enum ToHitreg {
    FromGui(GuiToHitreg),
    FromSerial(SerialToHitReg),
}

#[derive(Debug, Clone)]
pub enum ToGui {
    FromHitreg(HitregToGui),
    FromSerial(SerialToGui),
}
