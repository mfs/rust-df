use colored::*;
use regex::Regex;

// http://stackoverflow.com/questions/5194057/better-way-to-convert-file-sizes-in-python
pub fn iec(n: u64) -> String {
    let units = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];

    let i = (n as f64).log(1024_f64).floor() as u32;
    let p = 1024_u64.pow(i);
    let s = (n as f64)/(p as f64);
    format!("{:.0}{}", s, units[i as usize])
}

// /dev/mapper/vg-lv -> /dev/vg/lv
pub fn shorten_lv(path: &str) -> String {
    let re = Regex::new(r"^/dev/mapper/(.*?)-(.*)").unwrap();

    match re.captures(path) {
        Some(caps) => return format!("/dev/{}/{}", &caps[1], &caps[2].replace("--", "-")),
        None       => {},
    }

    path.to_string()
}

pub fn bargraph(percent: f64) -> String {
    let chars = "■■■■■■■■■■■■■■■■■■■■";
    let s1 = (percent / 10.0).round() as usize * 2;
    let s2 = 20 - s1;
    format!("{}{}", chars.chars().take(s1).collect::<String>().green(),
            chars.chars().take(s2).collect::<String>().white().dimmed())
}
