pub enum SerialToGuiKind {
    Reload,
    Shot,
}

pub struct SerialToGui {
    pub sensortag_id: u16,
    pub timestamp: u32,
    pub ammo: u8,
    pub ammo_max: u8,
    pub kind: SerialToGuiKind,
}

pub struct SerialToHitReg {
    pub id: u16,
    pub timestamp: u32,
    pub value_raw: u32,
}
