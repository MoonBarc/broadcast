//! gets target terminals to write to

use std::fs;
use std::os::unix::io::AsRawFd;

/// length, prefix
struct Target(i32, String);

pub fn get_ptys() -> std::io::Result<Vec<String>> {
    let mut ptys: Vec<String> = Vec::new();
    let paths = fs::read_dir("/dev/pts");
    if let Err(_) = paths {
        // some oses *cough* darwin *cough* dont have /dev/pts
        return Ok(Vec::new());
    }
    let paths = paths.unwrap();
    for p in paths {
        let pn = format!("{}",p.unwrap().path().display());
        if pn.contains("ptmx") {
            continue;
        };
        let pn = pn.replace("/dev/","");
        ptys.push(pn);
    }
    Ok(ptys)
}

/// gets tty or ttys in /dev directory
pub fn get_ttys() -> std::io::Result<Vec<String>> {
    let mut tty: Vec<String> = Vec::new();
    let paths = fs::read_dir("/dev")?;
    for p in paths {
        let prefixes = vec![
            Target(3, String::from("tty")),
            Target(3, String::from("ttys"))
        ];
        let pn = format!("{}", p.unwrap().path().display());
        for x in prefixes {
            let juice = pn.replace(
                format!("/dev/{}", x.1).as_str(),
                "");
            // ttys000 causes hang. idk why
            if juice.len() == 3 && pn != "/dev/ttys000" {
                tty.push(pn.replace("/dev/",""));
                break;
            }
        }
    }
    Ok(tty)
}

pub fn get_all() -> std::io::Result<Vec<String>> {
    let mut all = get_ptys()?;
    all.append(&mut get_ttys()?);
    // remove ourselves from the targets
    let us = nix::unistd::ttyname(std::io::stdout().as_raw_fd());
    if us.is_ok() {
        let us = us.unwrap().into_os_string().into_string().unwrap();
        all = all.into_iter()
            .filter(|t| *t != us.replace("/dev/",""))
            .collect();
    }
    Ok(all)
}