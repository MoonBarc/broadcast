use std::io::{Write/*, BufWriter*/};
use std::fs::{OpenOptions, File};
use termcolor::{Color, ColorSpec, Buffer, StandardStream, WriteColor};

use crate::write;
use crate::animation;
use crate::target;

pub fn send_all(stdout: &mut StandardStream, message: String, context: bool) -> std::io::Result<()> {
    let tty_names = target::get_all()?;
    let mut tty: Vec<File> = Vec::new();
    for t in tty_names {
        let file = OpenOptions::new().write(true).open(format!("/dev/{}",t));
        if let Err(_) = file {
            // println!("debug: failed opening write stream for a tty");
            // probably nothing to worry about...
            continue;
        }
        let file = file.unwrap();
        tty.push(file);
    }
    stdout.set_color(
        ColorSpec::new().set_bold(true).set_fg(
            Some(Color::Blue) // bold blue
        )
    )?;
    writeln!(stdout, "Broadcast v{}",env!("CARGO_PKG_VERSION"));
    stdout.set_color(
        ColorSpec::new().set_bold(false).set_fg(
            Some(Color::Blue) // just blue
        )
    )?;
    writeln!(stdout, "Drawing attention...");
    newline(&mut tty)?;
    warn(&mut tty)?;
    writeln!(stdout, "Broadcasting...");
    send(&message, &context, &mut tty)?;
    writeln!(stdout, "Done!");
    Ok(())
}

pub fn newline(tty: &mut Vec<File>) -> std::io::Result<()> {
    let mut recv_out = Buffer::ansi();
    write!(recv_out, "\n")?;
    for t in tty {
        write::write_recv_out(&mut recv_out, t);
    }
    Ok(())
}

pub fn warn(tty: &mut Vec<File>) -> std::io::Result<()> {
    let mut recv_out = Buffer::ansi();

    animation::inc_transmission(&mut recv_out, tty)?;
    Ok(())
}

pub fn send(message: &String, context: &bool, tty: &mut Vec<File>) -> std::io::Result<()> {
    let mut recv_out = Buffer::ansi();

    recv_out.set_color(
        ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true)
    )?;

    if !context {
        writeln!(&mut recv_out, "\n\n> {}\n", message);
    } else {
        recv_out.set_color(
            ColorSpec::new().set_fg(None).set_bold(true)
        )?;
        write!(&mut recv_out, "\n\nfrom: ");
        recv_out.set_color(
            ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)
        )?;
        writeln!(&mut recv_out, "{}", whoami::username());
        recv_out.set_color(
            ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true)
        )?;
        writeln!(&mut recv_out, "> {}\n", message);
    }

    recv_out.reset().expect("Failed to reset!");
    for t in tty {
        write::write_recv_out(&mut recv_out, t);
    }

    Ok(())
}
