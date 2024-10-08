use input_linux_sys::*;
use linux_raw_sys::ioctl::UI_DEV_CREATE;
use linux_raw_sys::ioctl::UI_DEV_DESTROY;
use linux_raw_sys::ioctl::UI_DEV_SETUP;
use std::ffi::CString;
use std::os::fd::AsFd;
use std::{fs::File, os::fd::AsRawFd};
use std::{mem, result, slice, thread, time};

// See C implementation
const UINPUT_IOCTL_BASE: u8 = b'U';
const UI_SET_EVBIT_P: u8 = 100;
const UI_SET_KEYBIT_P: u8 = 101;

/// Testing of ioctl() calls so as they are used in example in
/// https://kernel.org/doc/html/v4.19/input/uinput.html
#[macro_use]
ioctl_write_int!(ioctl_set_ebit, UINPUT_IOCTL_BASE, UI_SET_EVBIT_P);
ioctl_write_int!(ioctl_set_kbit, UINPUT_IOCTL_BASE, UI_SET_KEYBIT_P);
ioctl_write_ptr_bad!(ioclt_dev_setup, UI_DEV_SETUP, uinput_setup);
ioctl_none_bad!(ioctl_dev_create, UI_DEV_CREATE);
ioctl_none_bad!(ioctl_dev_destroy, UI_DEV_DESTROY);

const UINPUT_NAME: &str = "Test input device";
const UINPUT_NAME_SIZE: usize = 80;
const TEST_KEY: u16 = KEY_K as u16;

const REPORT: input_event = input_event {
    time: timeval { tv_sec: 0, tv_usec: 0 },
    type_: EV_SYN as u16,
    code: SYN_REPORT as u16,
    value: 0,
};
const KEY_EVENT: input_event = input_event {
    time: timeval { tv_sec: 0, tv_usec: 0 },
    type_: EV_KEY as u16,
    code: TEST_KEY, // Real code will be set later
    value: 1,       // Real value will be set later
};

unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub fn setup() -> result::Result<std::fs::File, &'static str> {
    // Prepare entries in uinput_setup. Must not be null
    let mut name: [i8; UINPUT_NAME_SIZE] = [0; UINPUT_NAME_SIZE];
    UINPUT_NAME.chars().enumerate().for_each(|(i, c)| {
        name[i] = c as i8;
    });
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

    // Setup
    let file = File::options().write(true).open("/dev/uinput").unwrap();
    unsafe {
        ioctl_set_ebit(file.as_raw_fd(), EV_KEY as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_ebit error");
        for i in 0..255 {
            ioctl_set_kbit(file.as_raw_fd(), i as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_kbit error");
        }
        ioclt_dev_setup(file.as_raw_fd(), &usetup).expect("ioclt_dev_setup error");
        ioctl_dev_create(file.as_raw_fd()).expect("ioctl_dev_create error");
        thread::sleep(time::Duration::new(1, 0));
    }
    Ok(file)
}

pub fn teardown(file: &std::fs::File) {
    // Teardown
    unsafe {
        thread::sleep(time::Duration::new(1, 0));
        ioctl_dev_destroy(file.as_raw_fd()).expect("ioctl_dev_destroy");
    }
    // Drop file
    let _ = file;
}

pub fn press_key(file: &std::fs::File, keycode: i32, pressed: bool) {
    let mut kevent = KEY_EVENT;
    kevent.value = if pressed == true { 1 } else { 0 };
    kevent.code = keycode as u16;

    let keyevt: &[u8] = unsafe { as_u8_slice(&kevent) };
    nix::unistd::write(file.as_fd(), &keyevt).expect("write KEY_EVENT failed");
    let report: &[u8] = unsafe { as_u8_slice(&REPORT) };
    nix::unistd::write(file.as_fd(), &report).expect("write REPORT failed");
}
