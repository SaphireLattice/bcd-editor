use uuid::Uuid;

// In reality, it's more of a "device" as in, a source of data. Virtual device more like
// More of a location definition, really
#[derive(Debug)]
pub struct Device {
    parent: Uuid,
    packet: DevicePacket
}

#[derive(Debug)]
pub struct PacketHeader {
    packet_type: u32,
    flags: u32,
    length: usize,
    unknown: u32
}

#[derive(Debug)]
pub enum DevicePacket {
    None,
    Boot,
    PartitionGPT(Uuid, Uuid),
    PartitionMBR(u32, u64),
    Path(u32, Box<DevicePacket>, String),
    Unknown(u32)
}

impl Device {
    pub fn new<'a>(raw: &Vec<u8>) -> Self {
        println!(" ({})", raw.len());
        let raw_uuid: [u8; 16] = raw[0..16].try_into().expect("slice with incorrect length");
        let first_uuid = Uuid::from_bytes_le(raw_uuid);

        if !first_uuid.is_nil() {
            /*let names = uuid_to_name(first_uuid);
            print!("        - Parent:    ");
            if names.0 == "" {
                println!("{}", names.1);
            } else {
                println!("{} ({})", names.1, names.0);
            }*/
            println!("        - Parent: {first_uuid}");
        }

        println!("        - Dump: {:02X?}", raw);

        return Device {
            parent: first_uuid,
            packet: Self::get_packet(&raw[16..])
        }
    }

    pub fn get_packet(raw: &[u8]) -> DevicePacket {
        let header = Self::get_packet_header(raw);

        if raw.len() != (header.length + if header.unknown != 0 { 4 } else { 0 }) {
            //println!("        - Leftover ({}): {:02X?}", raw.len() - skip - 16, &raw[skip + 16..]);
            //if skip == 0 {
            //    panic!("Unknown packet type: {} (continue: {})", packet_type, packet_flags);
            //}
            panic!("Packet does not match its declared size: actual {} vs declared {}", raw.len(), header.length)
        }

        //Vec::new()

        //more.push(Self::get_packet(packet_type, &raw[16..skip]));
        //more
        Self::get_packet_content(&raw[16..header.length], &header)
    }

    pub fn get_packet_header(raw: &[u8]) -> PacketHeader {
        println!("        - Header (len: {}): {:02X?}", raw.len(), &raw[0..16]);

        let length: usize = u32::from_le_bytes(raw[8..12].try_into().expect("invalid size")).try_into().unwrap();
        let unknown = u32::from_le_bytes(raw[12..16].try_into().expect("invalid size"));

        PacketHeader {
            packet_type: u32::from_le_bytes(raw[0..4].try_into().expect("invalid size")),
            flags: u32::from_le_bytes(raw[4..8].try_into().expect("invalid length")),
            length: length + if unknown != 0 { 4 } else { 0 },
            unknown
        }
    }

    pub fn get_packet_content(raw: &[u8], header: &PacketHeader) -> DevicePacket {
        println!("          - {:?} (actual len: {})", header, raw.len());

        if header.unknown == 5 {
            let nested_header = Self::get_packet_header(&raw[0..header.length - 16]);
            let filepath_raw = &raw[nested_header.length..header.length - 16];
            let filepath: Vec<u16> = filepath_raw
                .chunks_exact(2)
                .into_iter()
                .map(|a| u16::from_ne_bytes([a[0], a[1]]))
                .collect();
            let filepath = String::from_utf16(filepath.as_slice()).unwrap();
            println!("          - File: {}", filepath);
            let nested = Self::get_packet_content(&raw[16..nested_header.length], &nested_header);
            return DevicePacket::Path(header.packet_type, Box::new(nested), filepath);
        }

        match header.packet_type {
            0 => {
                let nested_start = if (header.flags == 0) == (header.unknown == 0) { 0 } else { 20 };
                if nested_start != 0 {
                    println!("          - probably some type: {}", u32::from_le_bytes(raw[0..4].try_into().expect("invalid size")));
                    println!("          - unknown: {:02X?}", &raw[4..nested_start]);
                }
                let nested_header = Self::get_packet_header(&raw[nested_start..header.length - 16]);
                let nested_end = nested_start + nested_header.length;
                Self::get_packet_content(&raw[nested_start + 16..nested_end], &nested_header);
                DevicePacket::None
            },
            5 => {
                assert!(raw.len() == 56);
                assert!(raw.iter().all(|&x| x == 0), "eh");
                DevicePacket::Boot
            },
            6 => {
                let partition_type = u32::from_le_bytes(raw[0x14..0x18].try_into().expect("invalid size"));
                println!("          - unknown: {:02X?}", &raw[0x10..0x14]);
                println!("          - unknown: {:02X?}", &raw[0x28..0x38]);
                let partition = match partition_type {
                    0 => DevicePacket::PartitionGPT(
                        Uuid::from_bytes_le(raw[0x18..0x28].try_into().expect("invalid size")),
                        Uuid::from_bytes_le(raw[0x00..0x10].try_into().expect("invalid size"))
                    ),
                    1 => DevicePacket::PartitionMBR(
                        u32::from_le_bytes(raw[0x18..0x1C].try_into().expect("invalid size")),
                        u64::from_le_bytes(raw[0x00..0x08].try_into().expect("invalid size"))
                    ),
                    invalid => panic!("Invalid partition type {}", invalid)
                };
                println!("          - partition: {:?}", partition);
                partition
            },
            unknown => DevicePacket::Unknown(unknown) //panic!("Unknown packet type: {}", unknown)
        }
    }


    /*pub fn other_new<'a>(raw: &Vec<u8>) -> Self {
        println!(" ({})", raw.len());
        let raw_uuid: [u8; 16] = raw[0..16].try_into().expect("slice with incorrect length");
        let first_uuid = Uuid::from_bytes_le(raw_uuid);

        let mut dev_offset = 0;
        if !first_uuid.is_nil() {
            /*let names = uuid_to_name(first_uuid);
            print!("        - Parent:    ");
            if names.0 == "" {
                println!("{}", names.1);
            } else {
                println!("{} ({})", names.1, names.0);
            }*/
            println!("        - Parent: {first_uuid}");
            dev_offset += 52;
        }

        print!("        - Device type: ");
        match u32::from_le_bytes(raw[dev_offset+16..dev_offset+20].try_into().expect("slice with incorrect length")) {
            6 => print!("partition"),
            default => print!("UNKNOWN DEVICE TYPE {}", default)
        }
        println!();
        println!("        - Packet device type: {}", u32::from_le_bytes(raw[16..20].try_into().expect("slice with incorrect length")));
        println!("        - Packet... nested?: {}", u32::from_le_bytes(raw[20..24].try_into().expect("slice with incorrect length")));
        let packet_size: usize = u32::from_le_bytes(raw[24..28].try_into().expect("slice with incorrect length")).try_into().expect("Unable to convert u32 into usize");
        println!("        - Packet size: {}", packet_size);
        println!("        - Header[3]: {}", u32::from_le_bytes(raw[28..32].try_into().expect("slice with incorrect length")));
        assert!(raw.len() == packet_size + 16, "Leftover data after the encoded packet");


        let dev_info = dev_offset + 32;
        println!("        - Device:    {}", Uuid::from_bytes_le(raw[dev_info+16+8..dev_info+16+8+16].try_into().expect("slice with incorrect length")));
        println!("        - Partition: {}", Uuid::from_bytes_le(raw[dev_info..dev_info+16].try_into().expect("slice with incorrect length")));

        if !first_uuid.is_nil() {
            let n = raw[60] - 86;
            let path_raw = raw[140 as usize..140 + n as usize].to_vec();
            let path: Vec<u16> = path_raw
                .chunks_exact(2)
                .into_iter()
                .map(|a| u16::from_ne_bytes([a[0], a[1]]))
                .collect();
            let path = String::from_utf16_lossy(path.as_slice());
            println!("        - Path: \"{}\"", path);
        }
        println!("{:02x?}", raw);

        Device {
            parent: first_uuid,
            packets: Vec::new()
        }
    }*/
}