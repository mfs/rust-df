extern crate nix;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use nix::sys::statvfs::vfs::Statvfs;

fn main() {
    let file = File::open("/proc/mounts").unwrap();
    let reader = BufReader::new(&file);

    let mut excludes = HashSet::new();
    let exclude_types = "cgroup autofs securityfs configfs pstore binfmt_misc debugfs \
                         hugetlbfs devpts mqueue proc sysfs";
    for t in exclude_types.split_whitespace() {
        excludes.insert(t);
    }

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if excludes.contains(fields[2]) {
                    continue;
                }
                let statvfs = Statvfs::for_path(fields[1]).unwrap();
                println!("{} {} {} {}", fields[0], fields[1], fields[2], statvfs.f_bsize);
            },
            Err(_) => panic!(),
        }
    }
}
