//! This is a translation of [Transmission's show tool](https://github.com/transmission/transmission/blob/master/utils/show.c)

/*
use std::ffi;
use std::ptr;
use std::os::raw;
use transmission_sys;

const MY_NAME: &str = "transmission-show";
const TIMEOUT_SECS: i32 = 30;

static options: &[transmission_sys::tr_option] = &[
    { val: 'm', longName: "magnet", description: "Give a magnet link for the specified torrent", shortName: "m", has_arg: 0, argName: ptr::null() },
    { val: 's', longName: "scrape", description: "Ask the torrent's trackers how many peers are in the torrent's swarm", shortName: "s", has_arg: 0, argName: ptr::null() },
    { val: 'V', longName: "version", description: "Show version number and exit", shortName: "V", has_arg: 0, argName: ptr::null() },
    { val: 0, longName: ptr::null(), description: ptr::null(), shortName: ptr::null(), has_arg: 0, argName: ptr::null() }
];

unsafe fn getUsage() -> ffi::CString {
    ffi::CString::new(format!("Usage: {} [options] <.torrent file>", MY_NAME))
}

unsafe fn parseCommandLine(argc: i32, argv: *const raw::c_char) -> i32 {
    let c: u8;
    let optarg: *const raw::c_char;

    c = transmission_sys::tr_getopt(getUsage(), argc, argv, options, &mut optarg);
    while c != transmission_sys::TR_OPT_DONE {
        match c as char {
            'm' => {},
            's' => {},
            'v' => {},
            transmission_sys::TR_OPT_UNK => {},
            _ => return 1
        }
        c = transmission_sys::tr_getopt(getUsage(), argc, argv, options, &mut optarg);
    }

    return 0;
}

unsafe fn doShowMagnet(inf: *const transmission_sys::tr_info) {
    let str = ffi::CStr::from_ptr(transmission_sys::tr_torrentInfoGetMagnetLink(inf));
    println!("{}", str);
}

unsafe fn compare_files_by_name() {
    unimplemented!();
}
*/

fn main() {
    unimplemented!();
}
