use colored::*;

// http://stackoverflow.com/questions/5194057/better-way-to-convert-file-sizes-in-python
pub fn iec(n: u64) -> String {
    let units = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];

    let i = (n as f64).log(1024_f64).floor() as u32;
    let p = 1024_u64.pow(i);
    let s = (n as f64)/(p as f64);
    format!("{:.0}{}", s, units[i as usize])
}

// valid VG/LV characters are: a-z A-Z 0-9 + _ . -
// we use this fact and replace -- with #
// split on - and then switch # back to -
pub fn shorten_lv(path: &str) -> String {
    const MARK: &'static str = "#";

    if path.starts_with("/dev/mapper/") {
        if let Some(lv) = path.split('/').nth(3) {
            let lv = lv.replace("--", MARK);
            let lv_vg: Vec<String> = lv.split("-").map(|x| x.replace(MARK, "-")).collect();
            return format!("/dev/{}/{}", lv_vg[0], lv_vg[1]);
        }
    }

    path.to_string()
}

pub fn bargraph(mut percent: f64) -> String {
    if percent.is_nan() {
        percent = 0.0;
    }
    let chars = "■■■■■■■■■■■■■■■■■■■■";
    let s1 = (percent / 10.0).round() as usize * 2;
    let s2 = 20 - s1;
    format!("{}{}", chars.chars().take(s1).collect::<String>().green(),
            chars.chars().take(s2).collect::<String>().white().dimmed())
}

#[cfg(test)]
mod tests {
    use super::shorten_lv;

    #[test]
    fn test_shorten_lv() {
        assert_eq!(shorten_lv("/dev/mapper/vg-lv"), "/dev/vg/lv");
        assert_eq!(shorten_lv("/dev/mapper/vg-lv--1"), "/dev/vg/lv-1");
        assert_eq!(shorten_lv("/dev/mapper/vg--one-lv"), "/dev/vg-one/lv");
        assert_eq!(shorten_lv("/dev/mapper/vg--one-lv--one"), "/dev/vg-one/lv-one");
    }
}
