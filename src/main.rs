mod element;
mod device;
use crate::device::Device;
use crate::element::Element;

use std::io::Read;
use std::fs::File;
use nt_hive::Hive;
use uuid::{Uuid};

fn main() {
    let mut buffer = Vec::new();

    let filename = std::env::args().nth(1).unwrap_or("BCD".to_string());

    File::open(filename).unwrap().read_to_end(&mut buffer).unwrap();

    let hive = Hive::new(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();

    let objects_node = root_key_node.subpath("Objects").unwrap().unwrap();


    println!("BCD Objects list:");

    for node in objects_node.subkeys().unwrap().unwrap() {
        //println!("");
        let node_actual = node.unwrap();
        let object_uuid_str = node_actual.name().unwrap().to_string_checked().unwrap();
        let object_uuid_name = object_uuid_to_name(object_uuid_str.clone());

        if object_uuid_name.0 == "" {
            println!("- {}", object_uuid_str);
        } else {
            println!("- {} ({})", object_uuid_name.1, object_uuid_name.0);
        }

        let description_node = node_actual.subpath("Description").unwrap().unwrap();
        let object_type = description_node.value("Type").unwrap().unwrap().dword_data().unwrap();

        print!("    ");
        match (object_type & 0xF000_0000) >> (4 * 7) {
            1 => {
                print!("Application object - ");
                match (object_type & 0x00F0_0000) >> (4 * 5) {
                    1 => print!("Firmware"),
                    2 => print!("Windows Boot"),
                    3 => print!("Legacy Loader"),
                    4 => print!("Real-mode Loader"),
                    default => print!("UNKNOWN APPLICATION {}", default)
                }
                print!(" - ");
                match object_type & 0x000F_FFFF {
                    1 => print!("fwbootmgr"),
                    2 => print!("bootmgr"),
                    3 => print!("osloader"),
                    4 => print!("resume"),
                    5 => print!("memdiag"),
                    6 => print!("ntldr"),
                    7 => print!("setupldr"),
                    8 => print!("bootsector"),
                    9 => print!("startup"),
                    10 => print!("bootapp"),
                    default => print!("UNKNOWN TYPE {}", default)
                }
            },
            2 => {
                print!("Inherit object - for ");
                match (object_type & 0x00F0_0000) >> (4 * 5) {
                    1 => print!("any objects"),
                    2 => print!("application objects"),
                    3 => print!("device objects"),
                    default => print!("UNKNOWN {}", default)
                }
            },
            3 => print!("Device object"),
            _ => print!("UNKNOWN OBJECT TYPE")
        }
        println!("");

        let elements_node = node_actual.subpath("Elements").unwrap().unwrap();

        if let Some(iterator) = elements_node.subkeys() {
            println!("    Elements:");
            for element in iterator.unwrap() {
                let element_actual = element.unwrap();
                Element::new(element_actual, object_type);
            }
        }

        println!("");
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
        return object_uuid_to_name(uuid.braced().to_string());
    }

    fn object_uuid_to_name(object_uuid_str: String) -> (String, String) {
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
