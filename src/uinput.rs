use input_linux_sys::*;
use linux_raw_sys::ioctl::{UI_DEV_CREATE, UI_DEV_DESTROY, UI_DEV_SETUP};
use std::ffi::CString;
use std::os::fd::AsFd;
use std::{fs::File, os::fd::AsRawFd};
use std::{result, slice, thread, time};

// See C implementation
const UINPUT_IOCTL_BASE: u8 = b'U';
const UI_SET_EVBIT_P: u8 = 100;
const UI_SET_KEYBIT_P: u8 = 101;
const UI_SET_RELBIT_P: u8 = 102;
const UI_SET_ABSBIT_P: u8 = 103;

/// Testing of ioctl() calls so as they are used in example in
/// https://kernel.org/doc/html/v4.19/input/uinput.html
#[macro_use]
ioctl_write_int!(ioctl_set_evbit, UINPUT_IOCTL_BASE, UI_SET_EVBIT_P);
ioctl_write_int!(ioctl_set_keybit, UINPUT_IOCTL_BASE, UI_SET_KEYBIT_P);
ioctl_write_int!(ioctl_set_relbit, UINPUT_IOCTL_BASE, UI_SET_RELBIT_P);
ioctl_write_int!(ioctl_set_absbit, UINPUT_IOCTL_BASE, UI_SET_ABSBIT_P);
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

const MOUSE_SET_EVENT: input_event = input_event {
    time: timeval { tv_sec: 0, tv_usec: 0 },
    type_: EV_ABS as u16,
    code: ABS_X as u16, // ABS_X or ABS_Y. Real code will be set later
    value: 1,           // Real value will be set later
};

const MOUSE_MOVE_EVENT: input_event = input_event {
    time: timeval { tv_sec: 0, tv_usec: 0 },
    type_: EV_REL as u16,
    code: REL_X as u16, // REL_X or REL_Y. Real code will be set later
    value: 1,           // Real value will be set later
};

unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub fn setup(display_width: i32, display_height: i32) -> result::Result<std::fs::File, &'static str> {
    // Prepare entries in uinput_user_dev. Must not be null
    let mut name: [i8; UINPUT_NAME_SIZE] = [0; UINPUT_NAME_SIZE];
    UINPUT_NAME.chars().enumerate().for_each(|(i, c)| {
        name[i] = c as i8;
    });
    let absmin: [i32; ABS_CNT as usize] = [0; ABS_CNT as usize];
    let mut absmax: [i32; ABS_CNT as usize] = [0; ABS_CNT as usize];
    absmax[ABS_X as usize] = display_width;
    absmax[ABS_Y as usize] = display_height;
    let absfuzz: [i32; ABS_CNT as usize] = [0; ABS_CNT as usize];
    let absflat: [i32; ABS_CNT as usize] = [0; ABS_CNT as usize];
    let userdev: uinput_user_dev = uinput_user_dev {
        id: input_id {
            bustype: BUS_USB,
            vendor: 0x1234,
            product: 0x5678,
            version: 0,
        },
        absmax,
        absmin,
        absfuzz,
        absflat,
        ff_effects_max: 0,
        name,
    };
    // Setup
    let file = File::options().write(true).open("/dev/uinput").unwrap();
    unsafe {
        // For keyboard
        ioctl_set_evbit(file.as_raw_fd(), EV_KEY as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_evbit for keyboard error");
        for i in 0..255 {
            ioctl_set_keybit(file.as_raw_fd(), i as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_keybit for keyboard error");
        }

        // For mouse
        ioctl_set_evbit(file.as_raw_fd(), EV_KEY as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_evbit for mouse error");
        ioctl_set_keybit(file.as_raw_fd(), BTN_LEFT as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_keybit for mouse error");
        ioctl_set_keybit(file.as_raw_fd(), BTN_RIGHT as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_keybit for mouse error");
        ioctl_set_evbit(file.as_raw_fd(), EV_ABS as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_evbit for mouse error");
        ioctl_set_absbit(file.as_raw_fd(), ABS_X as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_relbit for mouse error");
        ioctl_set_absbit(file.as_raw_fd(), ABS_Y as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_relbit for mouse error");
        // TODO: when this part with REL activated then the upper part with ABS does not work. Could not find why
        // ioctl_set_evbit(file.as_raw_fd(), EV_REL as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_evbit for mouse error");
        // ioctl_set_relbit(file.as_raw_fd(), REL_X as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_relbit for mouse error");
        // ioctl_set_relbit(file.as_raw_fd(), REL_Y as nix::sys::ioctl::ioctl_param_type).expect("ioctl_set_relbit for mouse error");

        // Finish
        let userdev_: &[u8] = unsafe { as_u8_slice(&userdev) };
        nix::unistd::write(file.as_fd(), &userdev_).expect("write uinput_user_dev failed");
        ioctl_dev_create(file.as_raw_fd()).expect("ioctl_dev_create error");
    }
    thread::sleep(time::Duration::new(1, 0));
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

pub fn press_key(file: &std::fs::File, keycode: i32, press: bool) {
    let mut kevent = KEY_EVENT;
    kevent.value = if press == true { 1 } else { 0 };
    kevent.code = keycode as u16;

    let keyevt: &[u8] = unsafe { as_u8_slice(&kevent) };
    nix::unistd::write(file.as_fd(), &keyevt).expect("write KEY_EVENT failed");
    let report: &[u8] = unsafe { as_u8_slice(&REPORT) };
    nix::unistd::write(file.as_fd(), &report).expect("write REPORT failed");
}

pub fn mouse_set(file: &std::fs::File, x: i32, y: i32) {
    let mut mevent: [input_event; 2] = [MOUSE_SET_EVENT, MOUSE_SET_EVENT];
    mevent[0].value = x;
    mevent[0].code = ABS_X as u16;
    mevent[1].value = y;
    mevent[1].code = ABS_Y as u16;

    let keyevt: &[u8] = unsafe { as_u8_slice(&mevent) };
    nix::unistd::write(file.as_fd(), &keyevt).expect("write MOUSE_EVENT Y failed");

    let report: &[u8] = unsafe { as_u8_slice(&REPORT) };
    nix::unistd::write(file.as_fd(), &report).expect("write REPORT failed");
}

pub fn mouse_move(file: &std::fs::File, x: i32, y: i32) {
    let mut mevent: [input_event; 2] = [MOUSE_MOVE_EVENT, MOUSE_MOVE_EVENT];
    mevent[0].value = x;
    mevent[0].code = REL_X as u16;
    mevent[1].value = y;
    mevent[1].code = REL_Y as u16;

    let keyevt: &[u8] = unsafe { as_u8_slice(&mevent) };
    nix::unistd::write(file.as_fd(), &keyevt).expect("write MOUSE_EVENT X failed");

    let report: &[u8] = unsafe { as_u8_slice(&REPORT) };
    nix::unistd::write(file.as_fd(), &report).expect("write REPORT failed");
}
