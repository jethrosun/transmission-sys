//# This is a translation of the [Transmission CLI](https://github.com/transmission/transmission/blob/master/cli/cli.c)

use std::ascii;
use transmission_sys;

type SigAtomicT = ::std::os::raw::c_int;

#[repr(C)]
pub struct tr_option<'a> {
    pub val:i32,
    pub longName:&'a str,
    pub description:&'a str,
    pub shortName:&'a str,
    pub has_arg:i32,
    pub argName:&'a str,
}

static SHOW_VERSION: bool = false;
static VERIFY: bool = false;
static GOTSIG: SigAtomicT = 0;
static MANUAL_UPDATE: SigAtomicT = 0;
static TORRENT_PATH: &str = "";
static options: [tr_option; 19] = [
    tr_option { val: 98, longName : "blocklist", description : "Enable peer blocklists", shortName : "b", has_arg : 0, argName : "" },
    tr_option { val: 66, longName : "no-blocklist", description : "Disable peer blocklists", shortName : "B", has_arg : 0, argName : "" },
    tr_option { val: 100, longName : "downlimit", description : "Set max download speed in kB/s", shortName : "d", has_arg : 1, argName : "<speed>" },
    tr_option { val: 68, longName : "no-downlimit", description : "Don't limit the download speed", shortName : "D", has_arg : 0, argName : "" },
    tr_option { val: 910, longName : "encryption-required", description : "Encrypt all peer connections", shortName : "er", has_arg : 0, argName : "" },
    tr_option { val: 911, longName : "encryption-preferred", description : "Prefer encrypted peer connections", shortName : "ep", has_arg : 0, argName : "" },
    tr_option { val: 912, longName : "encryption-tolerated", description : "Prefer unencrypted peer connections", shortName : "et", has_arg : 0, argName : "" },
    tr_option { val: 102, longName : "finish", description : "Run a script when the torrent finishes", shortName : "f", has_arg : 1, argName : "<script>" },
    tr_option { val: 103, longName : "config-dir", description : "Where to find configuration files", shortName : "g", has_arg : 1, argName : "<path>" },
    tr_option { val: 109, longName : "portmap", description : "Enable portmapping via NAT-PMP or UPnP", shortName : "m", has_arg : 0, argName : "" },
    tr_option { val: 77, longName : "no-portmap", description : "Disable portmapping", shortName : "M", has_arg : 0, argName : "" },
    tr_option { val: 112, longName : "port", description : "Port for incoming peers (Default: 51413)", shortName : "p", has_arg : 1, argName : "<port>" },
    tr_option { val: 116, longName : "tos", description : "Peer socket TOS (0 to 255, default= default)", shortName : "t", has_arg : 1, argName : "<tos>" },
    tr_option { val: 117, longName : "uplimit", description : "Set max upload speed in kB/s", shortName : "u", has_arg : 1, argName : "<speed>" },
    tr_option { val: 85, longName : "no-uplimit", description : "Don't limit the upload speed", shortName : "U", has_arg : 0, argName : "" },
    tr_option { val: 118, longName : "verify", description : "Verify the specified torrent", shortName : "v", has_arg : 0, argName : "" },
    tr_option { val: 86, longName : "version", description : "Show version number and exit", shortName : "V", has_arg : 0, argName : "" },
    tr_option { val: 119, longName : "download-dir", description : "Where to save downloaded data", shortName : "w", has_arg : 1, argName : "<path>" },
    tr_option { val: 0, longName : "", description : "", shortName : "", has_arg :0, argName : "" },

];
fn getUsage() -> String {
    return String::from("A fast and easy BitTorren client\n\nUsage: \" transmission-cli \" [options] <file|url|magnet>");
}

fn main() {
    println!("{}\n", getUsage());
}
