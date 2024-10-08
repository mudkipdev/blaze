use super::packet::{BasePacket, Deserializable, PacketType, Packet, PlayerIdentificationPacket};
use byteorder::ReadBytesExt;
use std::{
    io::{Cursor, Error, ErrorKind, Read},
    net::TcpStream,
};

pub struct Response {
    pub data: [u8; 1024],
}

impl Response {
    pub fn from_stream(stream: &mut TcpStream) -> Self {
        let mut data = [0; 1024];
        stream.read(&mut data).expect("Could't read stream data.");
        Self { data }
    }

    fn read_packet(&self) -> Result<Packet, Error> {
        let mut cursor = Cursor::new(self.data);
        let base = BasePacket {
            packet_type: PacketType::Upstream,
            packet_id: cursor.read_u8()?,
        };

        match base.packet_id {
            0x00 => Ok(Packet::PlayerIdentification(
                PlayerIdentificationPacket::build(cursor, base)?,
            )),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unknown packet ID")),
        }
    }

    pub fn to_packet(&self) -> Packet {
        self.read_packet().expect("Couldn't build packet.")
    }
}
