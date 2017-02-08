extern crate nix;

use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use nix::sys::statvfs::vfs::Statvfs;

// http://stackoverflow.com/questions/5194057/better-way-to-convert-file-sizes-in-python
fn iec(n: u64) -> String {
    let units = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];

    let i = (n as f64).log(1024_f64).floor() as u32;
    let p = 1024_u64.pow(i);
    let s = (n as f64)/(p as f64);
    format!("{:.0}{}", s, units[i as usize])
}

// /dev/mapper/vg-lv -> /dev/vg/lv
// this needs much better error checking
fn shorten_lv(path: &str) -> String {
    if path.starts_with("/dev/mapper/") {
        let lv = path.split('/').nth(3).unwrap();
        let c: Vec<&str> = lv.split('-').collect();
        return format!("/dev/{}/{}", c[0], c[1]);
    }
    path.to_string()
}

#[derive(Debug)]
struct Stats {
    filesystem: String,
    size: String,
    used: String,
    avail: String,
    mount: String,
}

fn main() {
    let file = File::open("/proc/mounts").unwrap();
    let reader = BufReader::new(&file);

    let mut excludes = HashSet::new();
    let exclude_types = "cgroup autofs securityfs configfs pstore binfmt_misc debugfs \
                         hugetlbfs devpts mqueue proc sysfs";
    for t in exclude_types.split_whitespace() {
        excludes.insert(t);
    }

    let mut stats: Vec<Stats> = Vec::new();
    let mut max_width = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if excludes.contains(fields[2]) {
                    continue;
                }
                let statvfs = Statvfs::for_path(fields[1]).unwrap();
                let size = statvfs.f_blocks * statvfs.f_bsize;
                let avail = statvfs.f_bavail * statvfs.f_bsize;
                let used = size - avail;
                //let percent =
                let s = Stats {
                    filesystem: shorten_lv(fields[0]),
                    size: iec(size),
                    used: iec(used),
                    avail: iec(avail),
                    mount: fields[1].to_string(),
                };
                max_width = cmp::max(max_width, s.filesystem.len());
                stats.push(s);
            },
            Err(_) => panic!(),
        }
    }

    let headers = ["Filesystem", "Size", "Used", "Avail", "Mounted on"];
    println!("{:width$} {:>5} {:>5} {:>5} {:16}",
             headers[0], headers[1], headers[2], headers[3],
             headers[4], width = max_width);

    for stat in stats {
        println!("{:width$} {:>5} {:>5} {:>5} {:16}",
                 stat.filesystem, stat.size, stat.used, stat.avail,
                 stat.mount, width = max_width);
    }

}
