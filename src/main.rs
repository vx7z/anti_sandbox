use sysinfo::{System, SystemExt, ProcessExt};
use windows::core::{s, PSTR};
use windows::Win32::System::SystemInformation::{
    GetSystemInfo, GlobalMemoryStatusEx, MEMORYSTATUSEX, SYSTEM_INFO,
};
use windows::Win32::System::Registry::{
    RegCloseKey, RegOpenKeyExA, RegQueryInfoKeyA, HKEY, HKEY_LOCAL_MACHINE, KEY_READ,
};
use std::time::Duration;
use std::thread;

fn main() {
    println!("Yo, let's see if we're in some fake computer world or whatever...");
    
    let mut sus_points = 0;
    
    sus_points += check_usb_stuff();
    sus_points += look_at_ram();
    sus_points += count_cpu_thingies();
    sus_points += see_whats_running();
    
    thread::sleep(Duration::from_secs(2));
    println!("\nOk, moment of truth...");
    thread::sleep(Duration::from_secs(1));
    
    if sus_points >= 2 {
        println!("Bruh, this is mad sus! Probs a fake computer. Sus level: {}", sus_points);
    } else {
        println!("Looks legit, I guess. Sus level: {}", sus_points);
    }
}

fn count_cpu_thingies() -> u32 {
    println!("Counting those CPU core thingies...");
    let mut info: SYSTEM_INFO = SYSTEM_INFO::default();
    unsafe {
        GetSystemInfo(&mut info);
    }
    if info.dwNumberOfProcessors < 2 {
        println!("LOL, only one core? What is this, a calculator?");
        1
    } else {
        println!("Cool, {} cores. Not too shabby!", info.dwNumberOfProcessors);
        0
    }
}

fn look_at_ram() -> u32 {
    println!("Checking out the RAM sitch...");
    let mut info: MEMORYSTATUSEX = MEMORYSTATUSEX::default();
    info.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
    unsafe {
        match GlobalMemoryStatusEx(&mut info) {
            Ok(_) => {
                let ram_gb = info.ullTotalPhys / (1024 * 1024 * 1024);
                if ram_gb <= 2 {
                    println!("Bruh, {}GB RAM? My toaster has more than that!", ram_gb);
                    1
                } else {
                    println!("{}GB RAM. Decent, I guess.", ram_gb);
                    0
                }
            },
            Err(_) => {
                println!("Ugh, can't even check the RAM. This computer's being weird.");
                0
            }
        }
    }
}

fn check_usb_stuff() -> u32 {
    println!("Checking out the USB situation...");
    let mut h_key: HKEY = HKEY::default();
    let mut usb_count: u32 = 0;
    let mut class_name_buffer = [0u8; 256];
    let mut class_name_length = class_name_buffer.len() as u32;

    unsafe {
        if RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            s!("SYSTEM\\ControlSet001\\Enum\\USBSTOR"),
            0,
            KEY_READ,
            &mut h_key,
        ).is_ok() {
            if RegQueryInfoKeyA(
                h_key,
                PSTR(class_name_buffer.as_mut_ptr()),
                Some(&mut class_name_length),
                None,
                Some(&mut usb_count),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ).is_ok() {
                if usb_count < 2 {
                    println!("Barely any USB stuff. This PC living under a rock or what?");
                    1
                } else {
                    println!("Found {} USB things. That's something, I guess.", usb_count);
                    0
                }
            } else {
                println!("Ugh, can't even count the USBs. This PC's being difficult.");
                0
            }
            let _ = RegCloseKey(h_key);
        } else {
            println!("Can't even look at USB stuff. This computer's got trust issues.");
            1
        }
    }
}

fn see_whats_running() -> u32 {
    println!("Let's see what's running on this thing...");
    let mut system = System::new_all();
    system.refresh_all();
    let process_count = system.processes().len();
    
    if process_count <= 50 {
        println!("Only {} things running? Is this PC even awake?", process_count);
        1
    } else {
        println!("Whoa, {} processes! This PC's busier than my group chat!", process_count);
        0
    }
}
