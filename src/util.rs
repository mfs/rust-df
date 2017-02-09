
// http://stackoverflow.com/questions/5194057/better-way-to-convert-file-sizes-in-python
pub fn iec(n: u64) -> String {
    let units = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];

    let i = (n as f64).log(1024_f64).floor() as u32;
    let p = 1024_u64.pow(i);
    let s = (n as f64)/(p as f64);
    format!("{:.0}{}", s, units[i as usize])
}

// /dev/mapper/vg-lv -> /dev/vg/lv
// this needs much better error checking
pub fn shorten_lv(path: &str) -> String {
    if path.starts_with("/dev/mapper/") {
        let lv = path.split('/').nth(3).unwrap();
        let c: Vec<&str> = lv.split('-').collect();
        return format!("/dev/{}/{}", c[0], c[1]);
    }
    path.to_string()
}
