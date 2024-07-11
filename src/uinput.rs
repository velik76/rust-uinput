use crate::test_scenario::TestScenario;
use input_linux_sys::*;
use libc::{c_void, fdopen, fwrite};
use linux_raw_sys::ioctl::UI_DEV_CREATE;
use linux_raw_sys::ioctl::UI_DEV_SETUP;
use linux_raw_sys::ioctl::UI_SET_EVBIT;
use linux_raw_sys::ioctl::UI_SET_KEYBIT;
use std::ffi::CString;
use std::{fs::File, os::fd::AsRawFd};

// #[macro_use]
// ioctl_write_ptr!(ui_dev_setup, UI_DEV_SETUP, 0, uinput_setup);
// ioctl_write_ptr(ui_dev_setup, UI_DEV_SETUP, EV_KEY);
// ioctl_write_int!(ui_ioctl_set_evbit, UI_SET_EVBIT, EV_KEY);
// ioctl_write_int!(ui_ioctl_set_keybit, UI_SET_KEYBIT, KEY_SPACE);

/// Testing of ioctl() calls so as they are used in example in
/// https://kernel.org/doc/html/v4.19/input/uinput.html
///
pub fn ioctl_test() {
    let file = File::open("/dev/uinput").unwrap();
    let fd = file.as_raw_fd();

    unsafe {
        if libc::ioctl(fd, u64::from(UI_SET_EVBIT), EV_KEY) < 0 {
            panic!("UI_SET_EVBIT");
        }
        if libc::ioctl(fd, u64::from(UI_SET_KEYBIT), KEY_K) < 0 {
            panic!("UI_SET_KEYBIT");
        }

        // Prepare entries in uinput_setup
        let mut usetup: uinput_setup = uinput_setup {
            id: input_id {
                bustype: BUS_USB,
                vendor: 0x1234,
                product: 0x5678,
                version: 0,
            },
            ff_effects_max: 0,
            name: [0; 80],
        };

        // To C const void*
        let ptr: *const uinput_setup = &usetup;
        let voidptr = ptr as *const c_void;
        if libc::ioctl(fd, u64::from(UI_DEV_SETUP), voidptr) < 0 {
            panic!("UI_DEV_SETUP");
        }
        if libc::ioctl(fd, u64::from(UI_DEV_CREATE)) < 0 {
            panic!("UI_DEV_CREATE");
        }
    }
}

fn get_keys_from_scenario(scenario: &TestScenario) -> Vec<u8> {
    let mut keys = Vec::new();
    for prog in &*scenario.program {
        if prog.get("type").unwrap() == "key_event" {
            let s_key = prog
                .get("param2")
                .unwrap()
                .to_string()
                .parse::<u8>()
                .unwrap();

            let mut not_exists = true;
            // for vkey in keys.iter() {
            //     if vkey == s_key {
            //         not_exists = false;
            //         break;
            //     }
            // }
            // keys.push(1);
        }
        print!("param1: {}. ", prog.get("param1").unwrap());
    }

    keys
}

pub fn init(scenario: &TestScenario) {
    //    let keys = get_keys_from_scenario(scenario);

    let mut file = File::open("/dev/uinput");
    let mut fd = 0;
    match file {
        Ok(filed) => fd = filed.as_raw_fd(),
        Err(err) => {
            println!("Error parsing of json file: {}", err);
        }
    }
    let mode = CString::new("w").unwrap();
    unsafe {
        let file = fdopen(fd, mode.as_ptr());
        if file.is_null() {
            panic!("can't open file");
        }
    }

    // unsafe {
    //     match ui_ioctl_set_evbit(fd, 0) {
    //         Err(err) => {
    //             println!("Error ui_ioctl_set_evbit(): {}", err);
    //         }
    //         Ok(_) => (),
    //     }

    //     match ui_ioctl_set_keybit(fd, 0) {
    //         Err(err) => {
    //             println!("Error ui_ioctl_set_keybit(): {}", err);
    //         }
    //         Ok(_) => (),
    //     }
    // }
    println!("All done");
}
