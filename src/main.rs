extern crate clap;
extern crate nix;
extern crate colored;

mod util;
mod stats;
mod filesystems;

use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process;
use nix::sys::statvfs::vfs::Statvfs;
use colored::*;
use clap::{Arg, App};

use util::{iec, bargraph};
use stats::Stats;
use filesystems::pseudo_filesystems;

const FS_SPEC: usize = 0;
const FS_FILE: usize = 1;
const FS_VFSTYPE: usize = 2;

fn main() {
    let matches = App::new("rdf")
        .version("0.1.0")
        .author("Mike Sampson <mike@sda.io>")
        .arg(Arg::with_name("all").long("all").short("a").help(
            "Display all filesystems",
        ))
        .get_matches();

    let file = match File::open("/proc/mounts") {
        Ok(f) => f,
        Err(e) => {
            println!("Error: Could not open /proc/mounts - {}", e);
            process::exit(1);
        }
    };
    let reader = BufReader::new(&file);

    let excludes = pseudo_filesystems();

    let mut stats: Vec<Stats> = Vec::new();
    let mut max_width = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if !matches.is_present("all") && excludes.contains(fields[FS_VFSTYPE]) {
                    continue;
                }
                let statvfs = match Statvfs::for_path(fields[FS_FILE]) {
                    Ok(s) => s,
                    Err(err) => {
                        println!("Error: {}", err);
                        continue;
                    }
                };
                let size = statvfs.f_blocks * statvfs.f_bsize;
                let avail = statvfs.f_bavail * statvfs.f_bsize;
                if size == 0 && !matches.is_present("all") {
                    continue;
                }
                let s = Stats::new(fields[FS_SPEC], size, avail, fields[FS_FILE]);

                max_width = cmp::max(max_width, s.filesystem.len());
                stats.push(s);
            }
            Err(err) => println!("Error: {}", err),
        }
    }

    stats.sort();

    let headers = [
        "Filesystem",
        "Size",
        "Used",
        "Avail",
        "Use%",
        "",
        "Mounted on",
    ];
    let headers: Vec<ColoredString> = headers.into_iter().map(|x| x.yellow()).collect();
    println!(
        "{:width$} {:>5} {:>5} {:>5} {:>5} {:20} {}",
        headers[0],
        headers[1],
        headers[2],
        headers[3],
        headers[4],
        headers[5],
        headers[6],
        width = max_width
    );

    for stat in stats {
        let fs = if stat.is_network() {
            stat.filesystem.cyan()
        } else {
            stat.filesystem.normal()
        };
        let percent = if stat.percent.is_nan() {
            "    -".to_string()
        } else {
            format!("{:>5.1}", stat.percent)
        };
        println!(
            "{:width$} {:>5} {:>5} {:>5} {} {:20} {}",
            fs,
            iec(stat.size),
            iec(stat.used),
            iec(stat.avail),
            percent,
            bargraph(stat.percent),
            stat.mount,
            width = max_width
        );
    }

}
