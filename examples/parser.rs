#[macro_use]
extern crate nom;

use nom::IResult;

#[derive(Debug, PartialEq)]
enum ProtocolVersion {
    V1,
    V2,
    Other(u8),
}

#[derive(Debug, PartialEq)]
struct PushData {
    version: ProtocolVersion,
    random_token: (u8, u8),
}

#[derive(Debug, PartialEq)]
struct PushAck {
    version: ProtocolVersion,
    random_token: (u8, u8),
}

#[derive(Debug, PartialEq)]
enum PacketType {
    PushData,
    PushAck,
}

#[derive(Debug, PartialEq)]
enum Packet {
    PushData(PushData),
    PushAck(PushAck),
}

/// Parse protocol version
named!(protocol_version<&[u8], ProtocolVersion>,
    map!(
        take!(1), |b: &[u8]| match b[0] {
            1 => ProtocolVersion::V1,
            2 => ProtocolVersion::V2,
            n @ _ => ProtocolVersion::Other(n),
        }
    )
);

/// Parse random token
named!(random_token<&[u8], (u8, u8)>,
    map!(
        take!(2), |b: &[u8]| (b[0], b[1])
    )
);

/// Parse packet type
named!(packet_type<&[u8], PacketType>,
    map_opt!(
        take!(1), |b: &[u8]| match b[0] {
            0x00 => Some(PacketType::PushData),
            0x01 => Some(PacketType::PushAck),
            _ => None,
        }
    )
);

named!(parse_packet<&[u8], Packet>,
    do_parse!(
        v: protocol_version >>
        r: random_token >>
        t: packet_type >>
        (match t {
            PacketType::PushData => Packet::PushData(PushData { version: v, random_token: r }),
            PacketType::PushAck => Packet::PushAck(PushAck { version: v, random_token: r }),
        })
    )
);

fn main() {
    let packet = vec![1, 39, 152, 0, 184, 39, 235, 255, 254, 74, 82, 19, 123,
    34, 115, 116, 97, 116, 34, 58, 123, 34, 116, 105, 109, 101, 34, 58, 34, 50,
    48, 49, 55, 45, 48, 50, 45, 48, 49, 32, 50, 49, 58, 52, 53, 58, 48, 54, 32,
    71, 77, 84, 34, 44, 34, 108, 97, 116, 105, 34, 58, 52, 55, 46, 50, 50, 56,
    52, 53, 44, 34, 108, 111, 110, 103, 34, 58, 56, 46, 56, 50, 57, 50, 52, 44,
    34, 97, 108, 116, 105, 34, 58, 52, 48, 57, 44, 34, 114, 120, 110, 98, 34,
    58, 49, 44, 34, 114, 120, 111, 107, 34, 58, 48, 44, 34, 114, 120, 102, 119,
    34, 58, 48, 44, 34, 97, 99, 107, 114, 34, 58, 54, 54, 46, 55, 44, 34, 100,
    119, 110, 98, 34, 58, 48, 44, 34, 116, 120, 110, 98, 34, 58, 48, 44, 34,
    112, 102, 114, 109, 34, 58, 34, 73, 77, 83, 84, 32, 43, 32, 82, 112, 105,
    34, 44, 34, 109, 97, 105, 108, 34, 58, 34, 100, 97, 110, 105, 108, 111, 64,
    99, 111, 114, 101, 100, 117, 109, 112, 46, 99, 104, 34, 44, 34, 100, 101,
    115, 99, 34, 58, 34, 82, 97, 112, 112, 101, 114, 115, 119, 105, 108, 32,
    71, 97, 116, 101, 119, 97, 121, 32, 98, 121, 32, 99, 111, 114, 101, 100,
    117, 109, 112, 46, 99, 104, 34, 125, 125];

    match parse_packet(&packet) {
        IResult::Done(i, o) => println!("Done: {:?}, Remaining: {:?}", o, i),
        IResult::Error(e) => println!("Error: {:?}", e),
        IResult::Incomplete(n) => println!("Needed more input: {:?}", n),
    };
}
