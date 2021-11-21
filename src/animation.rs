use termcolor::{Buffer, ColorSpec, Color, WriteColor};
use std::io::Write;
use std::fs::File;

use crate::write;

pub fn inc_transmission(recv_out: &mut Buffer, tty: &mut Vec<File>) -> std::io::Result<()> {
    for c in 0..20 {
        recv_out.set_color(
            ColorSpec::new().set_fg(Some(Color::Ansi256(7))).set_bold(true) //gray
        )?;
        let b = "...".to_string();
        let s = "   ".to_string();
        write!(recv_out, "\rIncoming transmission{}{}",&b[..(c%3+1)],&s[..(3-c%3)])?;
        recv_out.set_color(
            ColorSpec::new().set_fg(Some(Color::Ansi256(7))).set_bold(false) //gray
        )?;
        write!(recv_out, "({}) ", 5 - c / 4)?;
        recv_out.flush()?;
        for t in &mut *tty {
            write::write_recv_out(recv_out, t);
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    };
    write!(recv_out, "\rPrinting transmission...           ")?;
    for t in tty {
        write::write_recv_out(recv_out, t);
    }
    recv_out.reset()?;
    Ok(())
}