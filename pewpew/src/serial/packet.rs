use std::fmt::Debug;

pub const DELIMITER: u8 = 255;

#[derive(Debug, Copy, Clone)]
pub struct Packet {
    pub tag_id: u16,
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
                        tag_id,
                        timestamp,
                        content: PacketContent::Brightness(brightness),
                    })
                }
            }
            2 => {
                if value.len() != 8 {
                    Err(MessageParseError::InvalidPacketLength)
                } else {
                    Ok(Packet {
                        tag_id,
                        timestamp,
                        content: PacketContent::ButtonPressed,
                    })
                }
            }
            x => Err(MessageParseError::UnknownMessageCode(x)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PacketContent {
    ButtonPressed,
    Brightness(u16),
}
