extern crate getopts;
extern crate nix;
extern crate colored;

mod util;
mod stats;

use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process;
use std::collections::HashSet;
use nix::sys::statvfs::vfs::Statvfs;
use colored::*;
use std::env;
use getopts::Options;

use util::{iec, bargraph};
use stats::Stats;

const FS_SPEC: usize = 0;
const FS_FILE: usize = 1;
const FS_VFSTYPE: usize = 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("a", "all", "display all filesystems");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(e) => { panic!(e); },
    };

    let file = match File::open("/proc/mounts") {
        Ok(f)  => f,
        Err(e) => {
            println!("Error: Could not open /proc/mounts - {}", e);
            process::exit(1);
        },
    };
    let reader = BufReader::new(&file);

    let mut excludes = HashSet::new();
    let exclude_types = "cgroup autofs securityfs configfs pstore binfmt_misc debugfs \
                         hugetlbfs devpts mqueue proc sysfs fusectl gvfsd-fuse";
    if !matches.opt_present("a") {
        for t in exclude_types.split_whitespace() {
            excludes.insert(t);
        }
    }

    let mut stats: Vec<Stats> = Vec::new();
    let mut max_width = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if excludes.contains(fields[FS_VFSTYPE]) {
                    continue;
                }
                let statvfs = match Statvfs::for_path(fields[FS_FILE]) {
                    Ok(s) => s,
                    Err(err) => {
                        println!("Error: {}", err);
                        continue;
                    },
                };
                let size = statvfs.f_blocks * statvfs.f_bsize;
                let avail = statvfs.f_bavail * statvfs.f_bsize;
                if size == 0 && !matches.opt_present("a") {
                    continue;
                }
                let s = Stats::new(fields[FS_SPEC], size, avail, fields[FS_FILE]);

                max_width = cmp::max(max_width, s.filesystem.len());
                stats.push(s);
            },
            Err(err) => println!("Error: {}", err),
        }
    }

    stats.sort();

    let headers = ["Filesystem", "Size", "Used", "Avail", "Use%", "", "Mounted on"];
    let headers: Vec<ColoredString> = headers.into_iter().map(|x| x.yellow()).collect();
    println!("{:width$} {:>5} {:>5} {:>5} {:>5} {:20} {}",
             headers[0], headers[1], headers[2], headers[3],
             headers[4], headers[5], headers[6], width = max_width);

    for stat in stats {
        println!("{:width$} {:>5} {:>5} {:>5} {:>5.1} {:20} {}",
                 stat.filesystem, iec(stat.size), iec(stat.used), iec(stat.avail),
                 stat.percent, bargraph(stat.percent), stat.mount, width = max_width);
    }

}
