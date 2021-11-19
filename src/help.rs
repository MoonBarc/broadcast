use getopts::Options;

pub fn display_help(opts: &Options, progname: &String) {
    let brief = format!("Usage: {} [options] <message>", progname);
    print!("{}", opts.usage(&brief));
}