use std::str::FromStr;

use nt_hive::{KeyNode, Hive};
use uuid::{Uuid};

use crate::device::Device;


pub struct Element {
    id: u32,
    parent: u32,
    class: ElementClass,
    format: ElementFormat
}

pub enum ElementClass {
    Application,
    Inherit,
    Device,
    Unknown(u32)
}

pub enum ElementFormat {
    Device(Device),
    String(String),
    Guid(Uuid),
    GuidList(Vec<Uuid>),
    Integer(u64),
    Boolean(bool),
    IntegerList,
    Unknown(u32, Vec<u8>)
}

impl Element {
    pub fn new<'a>(key_node: KeyNode<&Hive<&'a [u8]>, &'a [u8]>, object_type: u32) -> Self {
        let element_name = key_node.name().unwrap().to_string_checked().unwrap();
        let element_type = u32::from_str_radix(element_name.as_str(), 16).unwrap();
        print!("    - {}", Self::element_header(element_type, object_type));

        /*if element_type == 0x12000005 {
            println!(" - String, LANG BUGGED ENTRY");
            return Element {
                id: element_type & 0x00FF_FFFF,
                parent: object_type,
                class: ElementClass::Application,
                format: ElementFormat::String("en_US".to_string())
            };
        }*/

        if let Some(value) = key_node.value("Element") {
            let unwrapped = value.unwrap();

            return Element {
                id: element_type & 0x00FF_FFFF,
                parent: object_type,
                class: ElementClass::Unknown((element_type & 0xF000_0000) >> (7 * 4)),
                format: match (element_type & 0x0F00_0000) >> (4 * 6) {
                    1 => {
                        let raw = unwrapped.data().unwrap().into_vec().unwrap();
                        let device = Device::new(&raw);
                        println!("{:?}", device);
                        ElementFormat::Device(device)
                    },
                    2 => {
                        let raw = unwrapped.string_data().unwrap();
                        println!(" = \"{}\"", raw);
                        ElementFormat::String(raw)
                    },
                    3 => {
                        let raw = Uuid::from_str(&unwrapped.string_data().unwrap()).unwrap();
                        println!(" = {}", raw);
                        ElementFormat::Guid(raw)
                    },
                    4 => {
                        println!();
                        let raw = unwrapped.multi_string_data().unwrap();
                        for guid in raw.clone() {
                            println!("        - {}", Self::name_to_str(Self::object_uuid_to_name(&guid.unwrap())));
                        }
                        ElementFormat::GuidList(raw.into_iter().map(|guid| Uuid::from_str(&guid.unwrap()).unwrap()).collect())
                    },
                    5 => {
                        let raw = unwrapped.data().unwrap().into_vec().unwrap();
                        println!(" = {:?}", raw);
                        ElementFormat::Integer(u64::from_le_bytes(raw[0..8].try_into().expect("slice with incorrect length")))
                    },
                    6 => {
                        let raw = unwrapped.data().unwrap().into_vec().unwrap();
                        println!(" = {}", if raw[0] == 0 { "false" } else { "true" });
                        ElementFormat::Boolean(raw[0] != 0)
                    },
                    default => {
                        println!(" = UNKNOWN({})", default);
                        ElementFormat::Unknown(default, [].to_vec())
                    }
                }
            };
        };

        panic!();
    }

    fn element_header(element_type: u32, object_type: u32) -> String {
        if object_type == 0 {
            panic!("aaa");
        }
        let matched = match element_type {
            0x1100_0001 => "device",
            0x1200_0002 => "path",
            0x1200_0004 => "description",
            0x1200_0005 => "locale",
            0x1400_0006 => "inherit",
            0x1500_0007 => "truncatememory",
            0x1400_0008 => "recoversequence",
            0x1600_0009 => "recoveryenabled",
            0x1700_000A => "badmemorylist",
            0x1600_000B => "badmemoryaccess",
            0x1500_000C => "firstmegabytepolicy",

            0x1600_0060 => "isolatedcontext",
            0x1700_0077 => "allowedinmemorysettings",

            0x3500_0001 => "ramdiskimageoffset",
            0x3500_0002 => "ramdisktftpclientport",
            0x3100_0003 => "ramdisksdidevice",
            0x3200_0004 => "ramdisksdipath",

            0x2500_00C2 => "bootmenupolicy",
            _ => ""
        };

        if matched != "" {
            return matched.to_owned();
        }

        let mut output = "".to_owned();
        match (element_type & 0xF000_0000) >> (4 * 7) {
            1 => output.push_str("Library"),
            2 => output.push_str("Application"),
            3 => output.push_str("Device"),
            4 => output.push_str("Template/OEM"),
            default => output.push_str(format!("UNKNOWN CLASS {}", default).as_str())
        }
        output.push_str(" (");
        match (element_type & 0x0F00_0000) >> (4 * 6) {
            1 => output.push_str("device"),
            2 => output.push_str("string"),
            3 => output.push_str("GUID"),
            4 => output.push_str("GUID list"),
            5 => output.push_str("integer"),
            6 => output.push_str("boolean"),
            7 => output.push_str("integer list"),
            default => output.push_str(format!("UNKNOWN FORMAT {}", default).as_str())
        }
        output.push_str(format!(") - {}", element_type & 0x00FF_FFFF).as_str());
        return output;
    }

    fn name_to_str(name: (String, String)) -> String {
        if name.1 == "" {
            return name.0;
        }
        if name.0 == "" {
            return name.1;
        }
        return format!("{} ({})", name.1, name.0);
    }

    fn uuid_to_name(uuid: Uuid) -> (String, String) {
        return Self::object_uuid_to_name(&uuid.braced().to_string());
    }

    fn object_uuid_to_name(object_uuid_str: &String) -> (String, String) {
        let object_uuid_str_upper = object_uuid_str.to_uppercase();
        let returning = match object_uuid_str_upper.as_str() {
            "{0CE4991B-E6B3-4B16-B23C-5E0D9250E5D9}" => ("GUID_EMS_SETTINGS_GROUP", "emssettings"),
            "{1AFA9C49-16AB-4A5C-4A90-212802DA9460}" => ("GUID_RESUME_LOADER_SETTINGS_GROUP", "resumeloadersettings"),
            "{1CAE1EB7-A0DF-4D4D-9851-4860E34EF535}" => ("GUID_DEFAULT_BOOT_ENTRY", "default"),
            "{313E8EED-7098-4586-A9BF-309C61F8D449}" => ("GUID_KERNEL_DEBUGGER_SETTINGS_GROUP", "kerneldbgsettings"),
            "{4636856E-540F-4170-A130-A84776F4C654}" => ("GUID_DEBUGGER_SETTINGS_GROUP", "dbgsettings"),
            "{466F5A88-0AF2-4F76-9038-095B170DC21C}" => ("WINDOWS_LEGACY_NTLDR", "ntldr"),
            "{5189B25C-5558-4BF2-BCA4-289B11BD29E2}" => ("BAD_MEMORY_GROUP", "badmemory"),
            "{6EFB52BF-1766-41DB-A6B3-0EE5EFF72BD7}" => ("BOOT_LOADER_SETTINGS_GROUP", "bootloadersettings"),
            "{7254A080-1510-4E85-AC0F-E7FB3D444736}" => ("GUID_WINDOWS_SETUP_EFI", ""),
            "{7EA2E1AC-2E61-4728-AAA3-896D9D0A9F0E}" => ("GUID_GLOBAL_SETTINGS_GROUP", "globalsettings"),
            "{7FF607E0-4395-11DB-B0DE-0800200C9A66}" => ("GUID_HYPERVISOR_SETTINGS_GROUP", "hypervisorsettings"),
            "{9DEA862C-5CDD-4E70-ACC1-F32B344D4795}" => ("GUID_WINDOWS_BOOTMGR", "bootmgr"),
            "{A1943BBC-EA85-487C-97C7-C9EDE908A38A}" => ("GUID_WINDOWS_OS_TARGET_TEMPLATE_PCAT", ""),
            "{A5A30FA2-3D06-4E9F-B5F4-A01DF9D1FCBA}" => ("GUID_FIRMWARE_BOOTMGR", "fwbootmgr"),
            "{AE5534E0-A924-466C-B836-758539A3EE3A}" => ("GUID_WINDOWS_SETUP_RAMDISK_OPTIONS", "ramdiskoptions"),
            "{B012B84D-C47C-4ED5-B722-C0C42163E569}" => ("GUID_WINDOWS_OS_TARGET_TEMPLATE_EFI", ""),
            "{B2721D73-1DB4-4C62-BF78-C548A880142D}" => ("GUID_WINDOWS_MEMORY_TESTER", "memdiag"),
            "{CBD971BF-B7B8-4885-951A-FA03044F5D71}" => ("GUID_WINDOWS_SETUP_PCAT", ""),
            "{FA926493-6F1C-4193-A414-58F0B2456D1E}" => ("GUID_CURRENT_BOOT_ENTRY", "current"),
            fallthrough => ("", fallthrough)
        };
        return (returning.0.to_owned(), returning.1.to_owned())
    }
}