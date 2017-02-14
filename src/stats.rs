use std::cmp::Ordering;

use util::shorten_lv;

#[derive(Debug)]
pub struct Stats {
    pub filesystem: String,
    pub size: u64,
    pub used: u64,
    pub avail: u64,
    pub percent: f64,
    pub mount: String,
}

impl Stats {
    pub fn new(fs: &str, size: u64, avail: u64, mount: &str) -> Stats {
        let used = size - avail;
        let percent = used as f64 / size as f64;
        Stats {
            filesystem: shorten_lv(fs),
            size: size,
            avail: avail,
            used: used,
            percent: 100.0 * percent,
            mount: mount.to_string(),
        }
    }
}

impl Ord for Stats {
    fn cmp(&self, other: &Stats) -> Ordering {
        self.filesystem.cmp(&other.filesystem)
    }
}

impl PartialOrd for Stats {
    fn partial_cmp(&self, other: &Stats) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Stats {
    fn eq(&self, other: &Stats) -> bool {
        self.filesystem == other.filesystem
    }
}

impl Eq for Stats {}
