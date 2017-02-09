
use util::shorten_lv;

#[derive(Debug)]
pub struct Stats {
    pub filesystem: String,
    pub size: u64,
    pub used: u64,
    pub avail: u64,
    pub mount: String,
}

impl Stats {
    pub fn new(fs: &str, size: u64, avail: u64, mount: &str) -> Stats {
        Stats {
            filesystem: shorten_lv(fs),
            size: size,
            avail: avail,
            used: size - avail,
            mount: mount.to_string(),
        }
    }
}
