use crate::serial::packet::MagazineStatus;

pub mod magazine;
pub mod scenery;

pub struct PlayerData {
    pub sensortag_id: u16,
    pub magazine_status: MagazineStatus,
    pub score: u32,
}