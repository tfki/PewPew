use std::fmt::Debug;

pub const DELIMITER: u8 = 255;

#[derive(Debug, Copy, Clone)]
pub struct Packet {
    pub sensortag_id: u16,
    pub timestamp: u32,
    pub content: PacketContent,
}

#[derive(Debug)]
pub enum MessageParseError {
    UnknownMessageCode(u8),
    InvalidPacketLength,
}

impl TryFrom<&[u8]> for Packet {
    type Error = MessageParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 && value.len() != 10 {
            return Err(MessageParseError::InvalidPacketLength);
        }

        let id_start = 0;
        let id_end = 2;

        let clock_start = 2;
        let clock_end = 6;

        let msg_type_start = 6;
        let msg_type_end = 7;

        let tag_id = u16::from_le_bytes(value[id_start..id_end].try_into().unwrap());
        let timestamp = u32::from_le_bytes(value[clock_start..clock_end].try_into().unwrap());
        let msg_type = u8::from_le_bytes(value[msg_type_start..msg_type_end].try_into().unwrap());

        match msg_type {
            1 => {
                if value.len() != 10 {
                    Err(MessageParseError::InvalidPacketLength)
                } else {
                    let brightness_start = 7;
                    let brightness_end = 9;
                    let brightness = u16::from_le_bytes(
                        value[brightness_start..brightness_end].try_into().unwrap(),
                    );

                    Ok(Packet {
                        sensortag_id: tag_id,
                        timestamp,
                        content: PacketContent::Brightness(brightness),
                    })
                }
            }
            2 => {
                if value.len() != 10 {
                    Err(MessageParseError::InvalidPacketLength)
                } else {
                    let ammo = u8::from_le(value[7]);
                    let ammo_max = u8::from_le(value[8]);
                    let mag_status = MagazineStatus{ammo, ammo_max};

                    Ok(Packet {
                        sensortag_id: tag_id,
                        timestamp,
                        content: PacketContent::ButtonPressed(mag_status),
                    })
                }
            }
            3 => {
                if value.len() != 10 {
                    Err(MessageParseError::InvalidPacketLength)
                } else {
                    let ammo = u8::from_le(value[7]);
                    let ammo_max = u8::from_le(value[8]);
                    let mag_status = MagazineStatus{ammo, ammo_max};

                    Ok(Packet {
                        sensortag_id: tag_id,
                        timestamp,
                        content: PacketContent::Reloaded(mag_status),
                    })
                }
            }
            x => Err(MessageParseError::UnknownMessageCode(x)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MagazineStatus {
    pub ammo: u8,
    pub ammo_max: u8,
}

#[derive(Debug, Copy, Clone)]
pub enum PacketContent {
    ButtonPressed(MagazineStatus),
    Brightness(u16),
    Reloaded(MagazineStatus),
}

#[cfg(test)]
mod tests {
    use crate::serial::packet::{MagazineStatus, MessageParseError, Packet, PacketContent};

    #[test]
    fn button_press_packets() {
        {
            let packet = Packet::try_from(
                [
                    0xFE_u8, 0xDC, // 2 bytes tag id (56574)
                    0x12, 0x34, 0x56, 0x78, // 4 bytes timestamp (2018915346)
                    0x02, // 1 bytes packet type
                    0x04, // bullets left
                    0x08, // mag size
                    0x00, // 1 byte for the delimiter, can be anything
                          // the packet does not parse the delimiter, it just expects a byte to be there
                ]
                .as_slice(),
            );
            assert!(matches!(
                packet,
                Ok(Packet {
                    sensortag_id: 56574,
                    timestamp: 2018915346,
                    content: PacketContent::ButtonPressed(MagazineStatus{ammo: 4, ammo_max: 8}),
                })
            ));
        }

        {
            let packet = Packet::try_from(
                [
                    0xCD_u8, 0xFE, // 2 bytes tag id (65229)
                    0x78, 0x56, 0x34, 0x12, // 4 bytes timestamp (305419896)
                    0x02, // 1 bytes packet type
                    0x00, // bullets left
                    0x08, // mag size
                    0xFF, // 1 byte for the delimiter, can be anything here
                          // the packet does not parse the delimiter, it just expects a byte to be there
                ]
                .as_slice(),
            );
            assert!(matches!(
                packet,
                Ok(Packet {
                    sensortag_id: 65229,
                    timestamp: 305419896,
                    content: PacketContent::ButtonPressed(MagazineStatus{ammo: 0, ammo_max: 8}),
                })
            ));
        }
    }

    #[test]
    fn brightness_packets() {
        {
            let packet = Packet::try_from(
                [
                    0xCD_u8, 0xFE, // 2 bytes tag id (65229)
                    0x78, 0x56, 0x34, 0x12, // 4 bytes timestamp (305419896)
                    0x01, // 1 bytes packet type
                    0xB0, 0x0B, // 2 bytes brightness value (2992)
                    0xFF, // 1 byte for the delimiter, can be anything here
                          // the packet does not parse the delimiter, it just expects a byte to be there
                ]
                .as_slice(),
            );
            assert!(matches!(
                packet,
                Ok(Packet {
                    sensortag_id: 65229,
                    timestamp: 305419896,
                    content: PacketContent::Brightness(2992),
                })
            ));
        }

        {
            let packet = Packet::try_from(
                [
                    0xCD_u8, 0xFE, // 2 bytes tag id (65229)
                    0x78, 0x56, 0x34, 0x12, // 4 bytes timestamp (305419896)
                    0x01, // 1 bytes packet type
                    0xBA, 0xAD, // 2 bytes brightness value (44474)
                    0xFF, // 1 byte for the delimiter, can be anything here
                          // the packet does not parse the delimiter, it just expects a byte to be there
                ]
                .as_slice(),
            );
            assert!(matches!(
                packet,
                Ok(Packet {
                    sensortag_id: 65229,
                    timestamp: 305419896,
                    content: PacketContent::Brightness(44474),
                })
            ));
        }
    }

    #[test]
    fn invalid_packet_types() {
        let invalid_codes = vec![0, /* 1, */ /* 2, */ 3, 4, 5, 6, 7, 8, 9, 10, 11];
        for code in invalid_codes {
            let packet = Packet::try_from(
                [
                    0xCD_u8, 0xFE, // 2 bytes tag id (65229)
                    0x78, 0x56, 0x34, 0x12, // 4 bytes timestamp (305419896)
                    code, // 1 bytes packet type THAT DOES NOT EXIST
                    0xBA, 0xAD, // 2 bytes brightness value (44474)
                    0xFF, // 1 byte for the delimiter, can be anything here
                          // the packet does not parse the delimiter, it just expects a byte to be there
                ]
                .as_slice(),
            );
            assert!(matches!(
                packet,
                Err(MessageParseError::UnknownMessageCode(c)) if c == code
            ));
        }
    }

    #[test]
    fn invalid_length_packets() {
        let invalid_packet_lengths = vec![
            0, 1, 2, 3, 4, 5, 6, 7, /* 8, */ 9, /* 10, */ 11, 12, 13, 14, 15, 16,
        ];
        for length in invalid_packet_lengths {
            let packet = Packet::try_from(vec![0_u8, length].as_slice());
            assert!(matches!(
                packet,
                Err(MessageParseError::InvalidPacketLength)
            ));
        }
    }
}
