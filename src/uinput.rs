use crate::test_scenario::TestScenario;
use input_linux_sys::*;
use libc::{c_int, c_ulong};
use libc::{c_void, fdopen, fwrite};
use linux_raw_sys::ioctl::UI_DEV_CREATE;
use linux_raw_sys::ioctl::UI_DEV_DESTROY;
use linux_raw_sys::ioctl::UI_DEV_SETUP;
use linux_raw_sys::ioctl::UI_SET_EVBIT;
use linux_raw_sys::ioctl::UI_SET_KEYBIT;
use std::ffi::CString;
use std::os::fd::AsFd;
use std::{fs::File, os::fd::AsRawFd};
use std::{mem, slice};
// #[macro_use]
// ioctl_write_ptr!(ui_dev_setup, UI_DEV_SETUP, 0, uinput_setup);
// ioctl_write_ptr(ui_dev_setup, UI_DEV_SETUP, EV_KEY);
// ioctl_write_int!(ui_ioctl_set_evbit, UI_SET_EVBIT, EV_KEY);
// ioctl_write_int!(ui_ioctl_set_keybit, UI_SET_KEYBIT, KEY_SPACE);

/// Testing of ioctl() calls so as they are used in example in
/// https://kernel.org/doc/html/v4.19/input/uinput.html
///
ioctl_write_ptr_bad!(set_uinput_setup, UI_DEV_SETUP, uinput_setup);
ioctl_none_bad!(ui_dev_create, UI_DEV_CREATE);
ioctl_none_bad!(ui_dev_destroy, UI_DEV_DESTROY);
ioctl_write_int!(ui_ioctl_set_evbit, UI_SET_EVBIT, EV_KEY);
ioctl_write_int!(ui_ioctl_set_keybit, UI_SET_KEYBIT, KEY_K);
const REPORT: input_event = input_event {
    time: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
    type_: EV_SYN as u16,
    code: SYN_REPORT as u16,
    value: 0,
};
const KEY_K_PRESSED: input_event = input_event {
    time: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
    type_: EV_KEY as u16,
    code: KEY_K as u16,
    value: 1,
};
const KEY_K_RELEASED: input_event = input_event {
    time: timeval {
        tv_sec: 0,
        tv_usec: 0,
    }, //16
    type_: EV_KEY as u16, //2
    code: KEY_K as u16,   //2
    value: 0,             //4
};

pub fn ioctl_test() {
    unsafe {
        let file = File::options().write(true).open("/dev/uinput").unwrap();
        // ui_ioctl_set_evbit(file.as_raw_fd(), EV_KEY  as nix::sys::ioctl::ioctl_param_type,).expect("set_evbits key");
        if libc::ioctl(file.as_raw_fd(), UI_SET_EVBIT as c_ulong, EV_KEY) < 0 {
            panic!("UI_SET_EVBIT");
        }
        // ui_ioctl_set_keybit(file.as_raw_fd(), KEY_K.try_into().unwrap()).expect("set_keybits k");
        if libc::ioctl(file.as_raw_fd(), UI_SET_KEYBIT as c_ulong, KEY_K) < 0 {
            panic!("UI_SET_KEYBIT");
        }

        // Prepare entries in uinput_setup
        // must not be null
        //let a = join!(b"Example device\0", [0; 79 - b"Example device\0".len()]);
        let mut v: [u8; 80] = [0; 80];
        let a = [0; 79 - b"Example device\0".len()];
        v[..b"Example device\0".len()].copy_from_slice(b"Example device\0");
        v[b"Example device\0".len() + 1..].copy_from_slice(&a);
        let name: [i8; 80] = unsafe { mem::transmute_copy(&v) };
        let usetup: uinput_setup = uinput_setup {
            id: input_id {
                bustype: BUS_USB,
                vendor: 0x1234,
                product: 0x5678,
                version: 0,
            },
            ff_effects_max: 0,
            name,
        };

        set_uinput_setup(file.as_raw_fd(), &usetup).expect("UI_DEV_SETUP");
        ui_dev_create(file.as_raw_fd()).expect("UI_DEV_CREATE");
        // let ret = libc::ioctl(file.as_raw_fd(), UI_DEV_CREATE as c_ulong);
        // println!("ret={:?}", ret);
        libc::sleep(1);
        // panic!("UI_DEV_CREATE");
        // libc::sleep(1);
        let key: [u8; 24] = unsafe { mem::transmute(KEY_K_PRESSED) };
        nix::unistd::write(file.as_fd(), &key).expect("write failed");

        let report: [u8; 24] = unsafe { mem::transmute(REPORT) };
        nix::unistd::write(file.as_fd(), &report).expect("write failed");

        let key: [u8; 24] = unsafe { mem::transmute(KEY_K_RELEASED) };
        nix::unistd::write(file.as_fd(), &key).expect("write failed");

        nix::unistd::write(file.as_fd(), &report).expect("write failed");

        libc::sleep(1);
        ui_dev_destroy(file.as_raw_fd()).expect("UI_DEV_DESTROY");
        // if libc::ioctl(file.as_raw_fd(), UI_DEV_DESTROY as c_ulong) < 0 {
        //     panic!("UI_DEV_CREATE");
        // }
        println!("\u{23CE}");
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
