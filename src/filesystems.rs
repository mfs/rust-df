use std::collections::HashSet;

// list of pseudo file systems
static PSEUDO: [&'static str; 17] = [
    "autofs",
    "binfmt_misc",
    "cgroup",
    "configfs",
    "debugfs",
    "devpts",
    "devtmpfs",
    "fusectl",
    "gvfsd-fuse",
    "hugetlbfs",
    "mqueue",
    "proc",
    "pstore",
    "run",
    "securityfs",
    "sysfs",
    "tmpfs",
];

pub fn pseudo_filesystems() -> HashSet<&'static str> {
    let mut filesystems = HashSet::new();
    for t in PSEUDO.iter() {
        filesystems.insert(*t);
    }

    filesystems
}

