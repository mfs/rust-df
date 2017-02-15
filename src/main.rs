extern crate nix;
extern crate colored;

mod util;
mod stats;

use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use nix::sys::statvfs::vfs::Statvfs;
use colored::*;

use util::{iec, bargraph};
use stats::Stats;

const FS_SPEC: usize = 0;
const FS_FILE: usize = 1;
const FS_VFSTYPE: usize = 2;

fn main() {
    let file = File::open("/proc/mounts").unwrap();
    let reader = BufReader::new(&file);

    let mut excludes = HashSet::new();
    let exclude_types = "cgroup autofs securityfs configfs pstore binfmt_misc debugfs \
                         hugetlbfs devpts mqueue proc sysfs fusectl gvfsd-fuse";
    for t in exclude_types.split_whitespace() {
        excludes.insert(t);
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
                let statvfs = Statvfs::for_path(fields[FS_FILE]).unwrap();
                let size = statvfs.f_blocks * statvfs.f_bsize;
                let avail = statvfs.f_bavail * statvfs.f_bsize;
                if size == 0 {
                    continue;
                }
                let s = Stats::new(fields[FS_SPEC], size, avail, fields[FS_FILE]);

                max_width = cmp::max(max_width, s.filesystem.len());
                stats.push(s);
            },
            Err(_) => panic!(),
        }
    }

    stats.sort();

    let headers = ["Filesystem", "Size", "Used", "Avail", "Use%", "", "Mounted on"];
    let headers: Vec<ColoredString> = headers.into_iter().map(|x| x.yellow()).collect();
    println!("{:width$} {:>5} {:>5} {:>5} {:>5} {:20} {:16}",
             headers[0], headers[1], headers[2], headers[3],
             headers[4], headers[5], headers[6], width = max_width);

    for stat in stats {
        println!("{:width$} {:>5} {:>5} {:>5} {:>5.1} {:20} {:16}",
                 stat.filesystem, iec(stat.size), iec(stat.used), iec(stat.avail),
                 stat.percent, bargraph(stat.percent), stat.mount, width = max_width);
    }

}
