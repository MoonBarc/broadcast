//! Broadcast messages to people/groups on a machine

use getopts::Options;
use std::{env, process};
use termcolor::{StandardStream, ColorChoice, WriteColor};

mod help;
mod send;
mod animation;
mod write;
mod target;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut context = true;

    // this program is linux only, does not need windows "console" support
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    let mut opts = Options::new();
    opts.optflag("h", "help",       "Displays this message and exits");
    opts.optflag("v", "version",    "Displays the current version and exits");
    opts.optflag("c", "no-context", "Removes context from the broadcast message");
    opts.optopt( "g", "group",      "Sends a message to a specific group", "NAME");
    opts.optflag("d", "",           "Debug something and exit!");
    let matches = opts.parse(&args[1..]);

    if let Err(getopts::Fail::UnrecognizedOption(a)) = matches {
        println!("Error: {} is not a valid option.", a);
        process::exit(1);
    } else if let Err(_) = matches {
        println!("Error parsing arguments!");
        process::exit(2);
    }

    let matches = matches.unwrap();

    if matches.opt_present("h") {
        help::display_help(&opts, &program);
        process::exit(0);
    }

    if matches.opt_present("d") {
        println!(
            "{:#?}",
            target::get_all().unwrap()
        );
        process::exit(0)
    }

    if matches.opt_present("v") {
        println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if matches.opt_present("c") {
        context = false;
    }

    if matches.free.len() == 0 {
        help::display_help(&opts, &program);
        process::exit(0);
    }

    let msg = matches.free.join(" ");
    send::send_all(&mut stdout, msg, context)?;
    stdout.reset()?;
    Ok(())
}